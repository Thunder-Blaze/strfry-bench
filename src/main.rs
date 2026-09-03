pub mod cli;
pub mod config;
pub mod system;
pub mod git;
pub mod builder;
pub mod relay;
pub mod workloads;
pub mod profiler;
pub mod analysis;
pub mod orchestrator;
pub mod web;

use clap::Parser;
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use secp256k1::{Keypair, Secp256k1};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use crate::cli::{Cli, Commands};
use crate::orchestrator::Orchestrator;
use crate::workloads::nostr::calculate_percentiles;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Check for legacy subcommands
    if let Some(cmd) = cli.command {
        return handle_legacy_command(cmd).await;
    }

    let config = cli.to_benchmark_config();

    if cli.web {
        let state = Arc::new(web::state::AppState::new(config.path.clone(), config.clone()));

        // If specific benchmark parameters were passed on CLI alongside --web,
        // automatically kick off the benchmark and stream live progress to the Web UI
        let should_auto_run = cli.test_type == crate::config::TestType::Compare
            || cli.flamegraph
            || cli.current
            || cli.base != "master"
            || cli.target != "HEAD";

        if should_auto_run {
            let state_clone = state.clone();
            let cfg_clone = config.clone();
            tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(600)).await;
                let progress_state = state_clone.clone();
                let progress = Arc::new(move |event: crate::orchestrator::ProgressEvent| {
                    match event {
                        crate::orchestrator::ProgressEvent::RunStarted { total_suites } => {
                            let mut status = progress_state.status.write();
                            status.is_running = true;
                            status.current_step = format!("Starting benchmark with {} suites", total_suites);
                            status.total_suites = total_suites;
                            status.completed_suites_count = 0;
                            status.current_suite = None;
                            progress_state.completed_suites.write().clear();
                        }
                        crate::orchestrator::ProgressEvent::Step { message } => {
                            progress_state.status.write().current_step = message;
                        }
                        crate::orchestrator::ProgressEvent::SuiteStarted { suite, index, total } => {
                            let mut status = progress_state.status.write();
                            status.current_suite = Some(suite.id().to_string());
                            status.current_step = format!("Testing {} ({}/{})", suite.display_name(), index, total);
                        }
                        crate::orchestrator::ProgressEvent::SuiteCompleted { result, completed, total, elapsed_secs, peak_rss_mb } => {
                            let mut suites = progress_state.completed_suites.write();
                            suites.push(result.clone());

                            let mut status = progress_state.status.write();
                            status.current_suite = None;
                            status.completed_suites_count = completed;
                            status.total_suites = total;
                            status.elapsed_secs = elapsed_secs;
                            if peak_rss_mb > status.peak_rss_mb {
                                status.peak_rss_mb = peak_rss_mb;
                            }
                            if let Some(tps) = result.throughput {
                                if tps > status.peak_tps {
                                    status.peak_tps = tps;
                                }
                            }
                            if let Some(p99) = result.p99_ms {
                                status.best_p99_ms = Some(status.best_p99_ms.map_or(p99, |old| old.min(p99)));
                            }
                            status.current_step = format!("Completed {} ({}/{})", result.name, completed, total);
                        }
                        crate::orchestrator::ProgressEvent::RunFinished { output_dir, elapsed_secs } => {
                            let mut status = progress_state.status.write();
                            status.is_running = false;
                            status.current_suite = None;
                            status.elapsed_secs = elapsed_secs;
                            status.current_step = format!("Completed! Reports saved to {}", output_dir.display());
                        }
                        crate::orchestrator::ProgressEvent::RunFailed { error } => {
                            let mut status = progress_state.status.write();
                            status.is_running = false;
                            status.current_suite = None;
                            status.current_step = format!("Failed: {}", error);
                        }
                    }
                });

                let orch = Orchestrator::new(cfg_clone)
                    .with_log_sender(state_clone.tx_log.clone())
                    .with_progress(progress);

                let start = std::time::Instant::now();
                let res = orch.run().await;
                let mut status = state_clone.status.write();
                status.is_running = false;
                status.elapsed_secs = start.elapsed().as_secs_f64();
                match res {
                    Ok(dir) => {
                        status.current_step = format!("Completed! Reports saved to {}", dir.display());
                    }
                    Err(e) => {
                        status.current_step = format!("Failed: {}", e);
                    }
                }
            });
        }

        web::start_web_server(cli.web_port, state).await?;
    } else {
        let orch = Orchestrator::new(config);
        match orch.run().await {
            Ok(out_dir) => {
                println!("\n[SUCCESS] Benchmark run completed successfully.");
                println!("[SUCCESS] Reports saved to: {}\n", out_dir.display());
            }
            Err(err) => {
                eprintln!("\n[ERROR] Benchmark run failed: {}\n", err);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

async fn handle_legacy_command(command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Event {
            url,
            concurrency,
            count,
            payload_size,
        } => {
            println!(
                "Starting Event benchmark against {} with {} connections, total {} events",
                url, concurrency, count
            );
            let (total, elapsed, lats) =
                workloads::event::execute_event_bench(&url, concurrency, count, payload_size).await;
            let tps = if elapsed > 0.0 { total as f64 / elapsed } else { 0.0 };
            println!("Sent {} events in {:.2?} ({:.2} events/sec)", total, elapsed, tps);
            print_percentiles(lats);
        }
        Commands::Req {
            url,
            concurrency,
            count,
            filter,
            nip45,
        } => {
            let label = if nip45 { "COUNT" } else { "REQ" };
            println!(
                "Starting {} benchmark against {} with {} connections, total {} reqs",
                label, url, concurrency, count
            );
            let (total, elapsed, lats) =
                workloads::req::execute_req_bench(&url, concurrency, count, &filter, nip45).await;
            let tps = if elapsed > 0.0 { total as f64 / elapsed } else { 0.0 };
            println!("Completed {} {}s in {:.2?} ({:.2} reqs/sec)", total, label, elapsed, tps);
            print_percentiles(lats);
        }
        Commands::Paginate {
            url,
            depth,
            concurrency,
        } => {
            println!("Starting Paginate benchmark against {} with depth {}", url, depth);
            let (pages, elapsed, _) =
                workloads::paginate::execute_paginate_bench(&url, depth, concurrency).await;
            println!("Paginate complete: {} pages in {:.2?}", pages, elapsed);
        }
        Commands::Monitor { url, subs, publish } => {
            println!(
                "Starting Monitor benchmark: {} subscriptions, {} events published",
                subs, publish
            );
            let (active, elapsed, first_lats, last_lats) =
                workloads::monitor::execute_monitor_bench(&url, subs, publish).await;
            println!("Confirmed {}/{} subs in {:.2?}", active, subs, elapsed);
            println!("Time-to-First-Client:");
            print_percentiles(first_lats);
            println!("Time-to-Last-Client:");
            print_percentiles(last_lats);
        }
        Commands::Connections { url, count } => {
            println!("Starting Connection storm benchmark: {} connections", count);
            let (total, elapsed, lats) =
                workloads::connections::execute_connections_bench(&url, count).await;
            let tps = if elapsed > 0.0 { total as f64 / elapsed } else { 0.0 };
            println!("Successfully established {} connections in {:.2?} ({:.2} conn/sec)", total, elapsed, tps);
            print_percentiles(lats);
        }
        Commands::Churn {
            url,
            concurrency,
            count,
        } => {
            println!("Starting Churn benchmark: {} total connections, {} concurrent", count, concurrency);
            let (total, elapsed, lats, close_lats) =
                workloads::churn::execute_churn_bench(&url, concurrency, count).await;
            let tps = if elapsed > 0.0 { total as f64 / elapsed } else { 0.0 };
            println!("Successfully churned {} connections in {:.2?} ({:.2} conn/sec)", total, elapsed, tps);
            println!("Total Connection Churn Latency:");
            print_percentiles(lats);
            println!("Socket Close Handshake Latency:");
            print_percentiles(close_lats);
        }
        Commands::Backpressure {
            url,
            fast_clients: _,
            slow_clients: _,
            count: _,
            slow_delay: _,
        } => {
            let res = workloads::backpressure::run_backpressure_suite(&url, false, &|msg| println!("{}", msg)).await;
            println!("{}", res.log_output);
        }
        Commands::Generate {
            count,
            authors,
            seed,
            payload_size: _,
        } => {
            let secp = Secp256k1::new();
            let mut rng = ChaCha8Rng::seed_from_u64(seed);

            let mut keypairs = Vec::with_capacity(authors);
            for _ in 0..authors {
                let mut secret_bytes = [0u8; 32];
                rng.fill(&mut secret_bytes);
                while secp256k1::SecretKey::from_byte_array(secret_bytes).is_err() {
                    rng.fill(&mut secret_bytes);
                }
                let secret = secp256k1::SecretKey::from_byte_array(secret_bytes).unwrap();
                let kp = Keypair::from_secret_key(&secp, &secret);
                keypairs.push(kp);
            }

            let kinds = [0, 1, 1, 1, 1, 3, 7, 30023];

            for _ in 0..count {
                let kp = &keypairs[rng.random_range(0..authors)];
                let kind = kinds[rng.random_range(0..kinds.len())];
                let content = format!("Deterministic bench event {}", rng.random::<u64>());
                let created_at = rng.random_range(1600000000..1800000000);
                let pubkey = kp.public_key().x_only_public_key().0.to_string();

                let mut event = workloads::nostr::Event {
                    id: String::new(),
                    pubkey,
                    created_at,
                    kind,
                    tags: vec![],
                    content,
                    sig: String::new(),
                };

                let serialized = serde_json::json!([
                    0,
                    event.pubkey,
                    event.created_at,
                    event.kind,
                    event.tags,
                    event.content
                ])
                .to_string();

                let mut hasher = Sha256::new();
                hasher.update(serialized.as_bytes());
                let id_bytes = hasher.finalize();
                event.id = hex::encode(id_bytes);

                let sig = secp.sign_schnorr_no_aux_rand(&id_bytes, kp);
                event.sig = sig.to_string();

                let json = serde_json::to_string(&event).unwrap();
                println!("{}", json);
            }
        }
        Commands::Malicious {
            url,
            count,
            slow_loris,
            sig_flood,
            duration,
        } => {
            println!("Starting Malicious client simulation against {} for {}s", url, duration);
            workloads::os_stress::execute_malicious_bench(&url, count, slow_loris, sig_flood, duration).await;
            println!("Malicious simulation complete.");
        }
    }

    Ok(())
}

fn print_percentiles(lats: Vec<u64>) {
    let (p50, p90, p95, p99) = calculate_percentiles(lats);
    if let (Some(p50), Some(p90), Some(p95), Some(p99)) = (p50, p90, p95, p99) {
        println!(
            "Latencies (ms) - P50: {:.2}, P90: {:.2}, P95: {:.2}, P99: {:.2}",
            p50, p90, p95, p99
        );
    }
}
