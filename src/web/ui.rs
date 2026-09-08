pub const ALPINE_JS: &str = include_str!("alpine.min.js");

pub const RENDERED_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Strfry Bench - Performance & Comparison Dashboard</title>
    <script defer src="/assets/alpine.min.js"></script>
    <style>
        :root {
            --bg-canvas: #090a0f;
            --bg-card: #10121a;
            --bg-card-hover: #141722;
            --border: #1a1d2e;
            --border-highlight: #2c324e;
            --text-primary: #f0f3f8;
            --text-secondary: #8c97ad;
            --text-muted: #535d73;
            --accent-lime: #c8f064;
            --accent-lime-bg: #16200c;
            --accent-cyan: #4ecdc4;
            --accent-cyan-bg: #0c2020;
            --accent-lavender: #8a99fc;
            --accent-lavender-bg: #191c33;
            --accent-red: #f87171;
            --accent-red-bg: #2b1414;
            --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            --font-mono: "JetBrains Mono", "Fira Code", monospace;
        }

        * { box-sizing: border-box; margin: 0; padding: 0; }

        ::-webkit-scrollbar { width: 5px; height: 5px; }
        ::-webkit-scrollbar-track { background: var(--bg-canvas); }
        ::-webkit-scrollbar-thumb { background: var(--border); border-radius: 2px; }
        ::-webkit-scrollbar-thumb:hover { background: var(--border-highlight); }

        body {
            background-color: var(--bg-canvas);
            color: var(--text-primary);
            font-family: var(--font-sans);
            line-height: 1.5;
            overflow-x: hidden;
            min-height: 100vh;
        }

        #app {
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            width: 100%;
        }

        .icon {
            width: 14px;
            height: 14px;
            min-width: 14px;
            min-height: 14px;
            stroke: currentColor;
            stroke-width: 2;
            fill: none;
            stroke-linecap: round;
            stroke-linejoin: round;
            vertical-align: middle;
            display: inline-block;
            flex-shrink: 0;
        }

        /* Navbar */
        .navbar {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 12px 32px;
            background: var(--bg-canvas);
            border-bottom: 1px solid var(--border);
            position: sticky;
            top: 0;
            z-index: 100;
        }

        .nav-left { display: flex; align-items: center; gap: 24px; }

        .brand-badge {
            background: var(--accent-lime);
            color: #090a0f;
            font-weight: 800;
            font-size: 12px;
            padding: 3px 8px;
            border-radius: 4px;
            letter-spacing: 0.5px;
            text-transform: uppercase;
        }

        .nav-links { display: flex; gap: 20px; }

        .nav-link {
            color: var(--text-secondary);
            font-size: 13px;
            font-weight: 600;
            cursor: pointer;
            text-decoration: none;
            padding: 4px 0;
            user-select: none;
            transition: color 0.15s ease;
        }

        .nav-link:hover, .nav-link.active {
            color: var(--text-primary);
            border-bottom: 2px solid var(--accent-lime);
        }

        .btn-ghost {
            background: transparent;
            border: 1px solid var(--border);
            color: var(--text-secondary);
            font-size: 11px;
            padding: 5px 10px;
            border-radius: 5px;
            cursor: pointer;
            display: inline-flex;
            align-items: center;
            gap: 5px;
            user-select: none;
            transition: all 0.15s ease;
        }

        .btn-ghost:hover, .btn-ghost.active {
            background: var(--bg-card);
            color: var(--text-primary);
            border-color: var(--border-highlight);
        }

        .container {
            max-width: 1540px;
            margin: 0 auto;
            padding: 24px 32px;
            width: 100%;
            display: grid;
            grid-template-columns: 1fr 380px;
            gap: 32px;
            flex: 1;
        }

        @media (max-width: 1200px) { .container { grid-template-columns: 1fr; } }

        .section-heading {
            font-family: var(--font-sans);
            font-size: 13px;
            font-weight: 700;
            color: var(--text-secondary);
            text-transform: uppercase;
            letter-spacing: 0.75px;
            margin-bottom: 10px;
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .page-title {
            font-size: 22px;
            font-weight: 700;
            letter-spacing: -0.5px;
            display: flex;
            align-items: center;
            gap: 10px;
        }

        /* 6 KPI Cards Single Row */
        .kpi-grid {
            display: grid;
            grid-template-columns: repeat(6, 1fr);
            gap: 12px;
            margin-bottom: 24px;
        }

        @media (max-width: 1100px) { .kpi-grid { grid-template-columns: repeat(3, 1fr); } }
        @media (max-width: 650px) { .kpi-grid { grid-template-columns: repeat(2, 1fr); } }

        .kpi-card {
            background: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: 6px;
            padding: 12px 14px;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            min-height: 92px;
            transition: border-color 0.15s ease;
        }

        .kpi-card:hover { border-color: var(--border-highlight); }

        .kpi-header {
            font-size: 11px;
            font-family: var(--font-sans);
            font-weight: 600;
            color: var(--text-secondary);
            text-transform: uppercase;
            letter-spacing: 0.5px;
            display: flex;
            align-items: center;
            gap: 6px;
        }

        .kpi-value {
            font-size: 24px;
            font-weight: 700;
            font-family: var(--font-mono);
            color: var(--text-primary);
            margin: 4px 0;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }

        .kpi-footer {
            font-size: 11px;
            color: var(--text-muted);
            font-family: var(--font-sans);
            font-weight: 500;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        /* Progress Bar */
        .progress-section { margin-bottom: 24px; }

        .progress-bar-bg {
            width: 100%;
            height: 5px;
            background: var(--bg-card);
            border-radius: 3px;
            overflow: hidden;
            margin-top: 6px;
            border: 1px solid var(--border);
        }

        .progress-bar-fill {
            height: 100%;
            background: var(--accent-lime);
            transition: width 0.3s ease;
        }

        /* Config Card */
        .config-card {
            background: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: 6px;
            padding: 16px 18px;
            margin-bottom: 24px;
        }

        .mode-toggle {
            display: inline-flex;
            background: #06070a;
            border: 1px solid var(--border);
            border-radius: 5px;
            padding: 2px;
            gap: 2px;
        }

        .mode-btn {
            background: transparent;
            border: none;
            color: var(--text-secondary);
            font-size: 11px;
            font-weight: 600;
            padding: 5px 12px;
            border-radius: 4px;
            cursor: pointer;
            user-select: none;
            transition: all 0.15s ease;
        }

        .mode-btn.active {
            background: var(--bg-card);
            color: var(--accent-lime);
        }

        .controls-row {
            display: grid;
            grid-template-columns: 1fr 1fr auto;
            gap: 14px;
            align-items: end;
        }

        @media (max-width: 900px) { .controls-row { grid-template-columns: 1fr; } }

        .control-group label {
            display: block;
            font-size: 10px;
            font-family: var(--font-mono);
            text-transform: uppercase;
            color: var(--text-muted);
            margin-bottom: 5px;
            font-weight: 600;
        }

        select, input[type="text"] {
            background: #06070a;
            border: 1px solid var(--border);
            color: var(--text-primary);
            padding: 7px 10px;
            border-radius: 5px;
            font-size: 12px;
            outline: none;
            width: 100%;
            transition: border-color 0.15s ease;
        }

        select:focus, input[type="text"]:focus { border-color: var(--accent-lime); }
        select:disabled, input[type="text"]:disabled { opacity: 0.5; cursor: not-allowed; }

        .btn-lime {
            background: var(--accent-lime);
            color: #090a0f;
            font-weight: 700;
            font-size: 12px;
            padding: 8px 20px;
            border: none;
            border-radius: 5px;
            cursor: pointer;
            display: inline-flex;
            align-items: center;
            gap: 6px;
            white-space: nowrap;
            user-select: none;
            transition: opacity 0.15s ease;
        }

        .btn-lime:hover { opacity: 0.9; }
        .btn-lime:disabled { opacity: 0.5; cursor: not-allowed; }

        .checkbox-label {
            display: flex;
            align-items: center;
            gap: 6px;
            font-size: 11px;
            color: var(--text-secondary);
            cursor: pointer;
            user-select: none;
        }

        /* Suite Cards */
        .suite-card {
            background: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: 6px;
            margin-bottom: 10px;
            overflow: hidden;
            transition: border-color 0.15s ease;
        }

        .suite-card:hover { border-color: var(--border-highlight); }
        .suite-card.running { border-color: var(--accent-cyan); }

        .suite-card-header {
            padding: 12px 16px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            cursor: pointer;
            user-select: none;
        }

        .badge {
            font-family: var(--font-mono);
            font-size: 10px;
            font-weight: 700;
            padding: 2px 6px;
            border-radius: 3px;
            text-transform: uppercase;
        }

        .badge.completed { background: var(--accent-lime-bg); color: var(--accent-lime); border: 1px solid rgba(200, 240, 100, 0.3); }
        .badge.running { background: var(--accent-cyan-bg); color: var(--accent-cyan); border: 1px solid rgba(78, 205, 196, 0.3); }
        .badge.queued { background: #06070a; color: var(--text-muted); border: 1px solid var(--border); }

        .suite-name { font-size: 13px; font-weight: 600; margin-left: 10px; }

        .suite-metrics-summary {
            font-family: var(--font-mono);
            font-size: 11px;
            color: var(--text-secondary);
            display: flex;
            gap: 12px;
            align-items: center;
        }

        .suite-collapse-body {
            padding: 14px 16px;
            border-top: 1px solid var(--border);
            background: #06070a;
        }

        /* Dynamic Stats Grid (Auto-fits active panels) */
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
            gap: 12px;
            margin-bottom: 14px;
        }

        .stat-panel {
            background: #0b0d13;
            border: 1px solid var(--border);
            border-radius: 5px;
            padding: 10px 14px;
        }

        .stat-panel-title {
            font-size: 10px;
            font-family: var(--font-sans);
            font-weight: 700;
            color: var(--text-secondary);
            text-transform: uppercase;
            letter-spacing: 0.5px;
            margin-bottom: 8px;
        }

        /* Markdown Rendered Output */
        .md-rendered-output {
            background: #06070a;
            border: 1px solid var(--border);
            border-radius: 5px;
            padding: 12px 14px;
            font-size: 12px;
            line-height: 1.6;
        }

        .md-kv-row {
            display: flex;
            align-items: baseline;
            gap: 8px;
            margin-bottom: 4px;
            font-family: var(--font-mono);
            font-size: 11.5px;
        }

        .md-key {
            color: var(--text-muted);
            font-weight: 600;
            min-width: 170px;
        }

        .md-val {
            color: var(--text-primary);
            font-weight: 500;
        }

        .md-heading {
            font-size: 12px;
            font-weight: 700;
            color: var(--accent-lavender);
            text-transform: uppercase;
            letter-spacing: 0.5px;
            margin: 10px 0 6px 0;
            border-bottom: 1px solid rgba(138, 153, 252, 0.2);
            padding-bottom: 3px;
        }

        .md-code-block {
            background: #030407;
            border: 1px solid var(--border);
            border-radius: 4px;
            padding: 8px 12px;
            font-family: var(--font-mono);
            font-size: 11px;
            color: #a0aec0;
            line-height: 1.5;
            margin: 6px 0;
            white-space: pre-wrap;
            word-break: break-all;
        }

        .md-bullet-item {
            color: var(--text-secondary);
            font-size: 12px;
            margin-bottom: 3px;
        }

        .md-bullet {
            color: var(--accent-lime);
            margin-right: 4px;
        }

        .diff-table {
            width: 100%;
            border-collapse: collapse;
            font-size: 12px;
        }

        .diff-table th, .diff-table td {
            padding: 8px 12px;
            border-bottom: 1px solid var(--border);
            text-align: left;
        }

        .diff-table th { font-family: var(--font-mono); font-size: 10px; color: var(--text-muted); text-transform: uppercase; }
        .diff-table tr:hover td { background: var(--bg-card-hover); }

        .delta-badge {
            font-family: var(--font-mono);
            font-size: 11px;
            font-weight: 700;
            padding: 3px 8px;
            border-radius: 4px;
            display: inline-block;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        .delta-badge.improved {
            background: var(--accent-lime-bg);
            color: var(--accent-lime);
            border: 1px solid rgba(200, 240, 100, 0.4);
        }

        .delta-badge.regressed {
            background: var(--accent-red-bg);
            color: var(--accent-red);
            border: 1px solid rgba(248, 113, 113, 0.4);
        }

        .delta-badge.stable {
            background: #11141f;
            color: var(--text-secondary);
            border: 1px solid var(--border);
        }

        .delta-badge.completed {
            background: #11141f;
            color: var(--accent-lime);
            border: 1px solid var(--border);
        }

        /* Multi-Quantile Spectrum Bar */
        .quantile-bar {
            display: flex;
            height: 7px;
            border-radius: 3px;
            overflow: hidden;
            border: 1px solid var(--border);
            background: #06070a;
            margin-top: 4px;
        }

        /* Flamegraph Container */
        .flamegraph-viewer {
            width: 100%;
            max-width: 1240px;
            margin: 0 auto;
            border: 1px solid var(--border);
            border-radius: 6px;
            background: #06070a;
            display: flex;
            flex-direction: column;
        }

        .flamegraph-toolbar {
            padding: 12px 18px;
            background: var(--bg-card);
            border-bottom: 1px solid var(--border);
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 14px;
            flex-wrap: wrap;
        }

        .flamegraph-frame {
            width: 100%;
            border: none;
            background: #ffffff;
            display: block;
            min-height: 700px;
        }
        /* Sidebar */
        .sidebar-card {
            background: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: 6px;
            padding: 16px;
            display: flex;
            flex-direction: column;
            height: 100%;
        }

        .log-terminal {
            background: #06070a;
            border: 1px solid var(--border);
            border-radius: 5px;
            padding: 10px;
            font-family: var(--font-mono);
            font-size: 11px;
            color: #8c97ad;
            height: 500px;
            overflow-y: auto;
            white-space: pre-wrap;
            word-break: break-all;
        }

        .footer {
            border-top: 1px solid var(--border);
            padding: 10px 32px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            font-size: 11px;
            color: var(--text-muted);
            font-family: var(--font-mono);
            background: var(--bg-canvas);
            margin-top: auto;
            width: 100%;
        }
    </style>
</head>
<body>
    <div id="app" x-data="benchApp()">
        <!-- Top Navigation Bar -->
        <nav class="navbar">
            <div class="nav-left">
                <div class="brand-badge">STRFRY</div>
                <div class="nav-links">
                    <a class="nav-link" :class="activeTab === 'runner' ? 'active' : ''" @click="activeTab = 'runner'">BENCHMARK</a>
                    <a class="nav-link" :class="activeTab === 'comparison' ? 'active' : ''" @click="openComparisonTab()">COMPARISON</a>
                    <a class="nav-link" :class="activeTab === 'past' ? 'active' : ''" @click="openPastTab()">PAST RUNS</a>
                    <a class="nav-link" :class="activeTab === 'analytics' ? 'active' : ''" @click="openAnalyticsTab()">ANALYTICS</a>
                    <a class="nav-link" :class="activeTab === 'flamegraph' ? 'active' : ''" @click="openFlamegraphTab()">FLAMEGRAPH</a>
                </div>
            </div>
            <div style="display: flex; align-items: center; gap: 12px;">
                <span style="display: flex; align-items: center; gap: 6px; font-size: 11px; font-family: var(--font-mono); color: var(--text-secondary);">
                    <span style="width: 6px; height: 6px; border-radius: 50%;" :style="'background: ' + (wsConnected ? 'var(--accent-lime)' : 'var(--accent-red)')"></span>
                    <span x-text="wsConnected ? 'CONNECTED :7787' : 'CONNECTING...'"></span>
                </span>
                <button class="btn-ghost" @click="refreshStatus()">
                    <svg class="icon" viewBox="0 0 24 24"><path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/><path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"/><path d="M16 21h5v-5"/></svg>
                    REFRESH
                </button>
            </div>
        </nav>

        <div class="container" :style="activeTab !== 'runner' ? 'grid-template-columns: 1fr; max-width: 1440px;' : ''">
            <main>
                <!-- KPI Row -->
                <div class="kpi-grid">
                    <div class="kpi-card">
                        <div class="kpi-header">
                            <svg class="icon" viewBox="0 0 24 24"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
                            COMPLETED
                        </div>
                        <div class="kpi-value" x-text="status.completed_suites_count + ' / ' + status.total_suites"></div>
                        <div class="kpi-footer">SUITES DONE</div>
                    </div>
                    <div class="kpi-card">
                        <div class="kpi-header">
                            <svg class="icon" viewBox="0 0 24 24" style="color: var(--accent-cyan);"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg>
                            ACTIVE TEST
                        </div>
                        <div class="kpi-value" style="font-size: 13px; color: var(--accent-cyan);" x-text="status.current_suite || (status.is_running ? 'RUNNING' : 'IDLE')"></div>
                        <div class="kpi-footer" x-text="status.is_running ? 'IN PROGRESS' : 'AWAITING RUN'"></div>
                    </div>
                    <div class="kpi-card">
                        <div class="kpi-header">
                            <svg class="icon" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
                            DURATION
                        </div>
                        <div class="kpi-value" x-text="status.elapsed_secs.toFixed(1) + 's'"></div>
                        <div class="kpi-footer">TOTAL WALL TIME</div>
                    </div>
                    <div class="kpi-card">
                        <div class="kpi-header">
                            <svg class="icon" viewBox="0 0 24 24" style="color: var(--accent-lime);"><polyline points="23 6 13.5 15.5 8.5 10.5 1 18"/><polyline points="17 6 23 6 23 12"/></svg>
                            PEAK RATE
                        </div>
                        <div class="kpi-value" style="color: var(--accent-lime);" x-text="status.peak_tps > 0 ? Math.round(status.peak_tps).toLocaleString() : '-'"></div>
                        <div class="kpi-footer">MAX THROUGHPUT</div>
                    </div>
                    <div class="kpi-card">
                        <div class="kpi-header">
                            <svg class="icon" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"/><path d="m4.93 4.93 4.24 4.24"/><path d="m14.83 9.17 4.24-4.24"/><path d="m14.83 14.83 4.24 4.24"/><path d="m9.17 14.83-4.24 4.24"/></svg>
                            BEST P99
                        </div>
                        <div class="kpi-value" x-text="status.best_p99_ms ? status.best_p99_ms.toFixed(2) + 'ms' : '-'"></div>
                        <div class="kpi-footer">QUERY ENGINE</div>
                    </div>
                    <div class="kpi-card">
                        <div class="kpi-header">
                            <svg class="icon" viewBox="0 0 24 24" style="color: var(--accent-lavender);"><rect width="20" height="8" x="2" y="2" rx="2"/><rect width="20" height="8" x="2" y="14" rx="2"/><line x1="6" x2="6.01" y1="6" y2="6"/><line x1="6" x2="6.01" y1="18" y2="18"/></svg>
                            MEMORY
                        </div>
                        <div class="kpi-value" style="color: var(--accent-lavender);" x-text="status.peak_rss_mb > 0 ? status.peak_rss_mb.toFixed(1) + ' MB' : '-'"></div>
                        <div class="kpi-footer">PROCESS VMRSS</div>
                    </div>
                </div>

                <!-- 1. BENCHMARK RUNNER VIEW -->
                <div x-show="activeTab === 'runner'">
                    <!-- Progress Bar Section -->
                    <div class="progress-section">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                            <div class="section-heading" style="margin-bottom: 0;">Live Progress</div>
                            <div style="font-family: var(--font-mono); font-size: 12px; color: var(--text-secondary);" x-text="Math.round((status.completed_suites_count / status.total_suites) * 100) + '% completed · ' + status.current_step"></div>
                        </div>
                        <div class="progress-bar-bg">
                            <div class="progress-bar-fill" :style="'width: ' + Math.round((status.completed_suites_count / status.total_suites) * 100) + '%'"></div>
                        </div>
                    </div>

                    <!-- Controls Card -->
                    <div class="config-card">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 14px; flex-wrap: wrap; gap: 10px;">
                            <div style="display: flex; gap: 8px;">
                                <div class="mode-toggle">
                                    <button class="mode-btn" :class="currentTargetMode === 'source' ? 'active' : ''" @click="currentTargetMode = 'source'">Source Build</button>
                                    <button class="mode-btn" :class="currentTargetMode === 'live' ? 'active' : ''" @click="currentTargetMode = 'live'">Live Relay</button>
                                </div>
                                <div class="mode-toggle" x-show="currentTargetMode === 'source'">
                                    <button class="mode-btn" :class="currentTestMode === 'single' ? 'active' : ''" @click="currentTestMode = 'single'">Single Test</button>
                                    <button class="mode-btn" :class="currentTestMode === 'compare' ? 'active' : ''" @click="currentTestMode = 'compare'">Comparison A/B</button>
                                </div>
                            </div>
                            <div style="font-size: 12px; font-weight: 500; color: var(--text-muted); font-family: var(--font-mono);">Target: <span style="color: var(--accent-lime);" x-text="repoPath || defaultUrl"></span></div>
                        </div>

                        <!-- Source Build Controls -->
                        <div x-show="currentTargetMode === 'source'" class="controls-row">
                            <!-- Initial Base -->
                            <div class="control-group" x-show="currentTestMode === 'compare'">
                                <label>Initial (Base Branch & Commit)</label>
                                <div style="display: flex; gap: 6px;">
                                    <select x-model="selectedBaseBranch" @change="onBaseBranchChanged()" style="width: 130px;">
                                        <template x-for="b in branches" :key="b"><option :value="b" x-text="b"></option></template>
                                    </select>
                                    <select x-model="selectedBaseCommit">
                                        <template x-for="c in baseCommits" :key="c.hash"><option :value="c.hash" x-text="c.short_hash + ' - ' + c.message.substring(0, 28)"></option></template>
                                    </select>
                                </div>
                            </div>

                            <!-- Final Target -->
                            <div class="control-group">
                                <label x-text="currentTestMode === 'compare' ? (compareCurrent ? 'Final: Current Codebase (Working Tree)' : 'Final (Target Branch & Commit)') : (compareCurrent ? 'Target: Current Codebase (Working Tree)' : 'Branch & Commit to Benchmark')"></label>
                                <div style="display: flex; gap: 6px;">
                                    <select x-model="selectedTargetBranch" @change="onTargetBranchChanged()" :disabled="compareCurrent" style="width: 130px;">
                                        <template x-for="b in branches" :key="b"><option :value="b" x-text="b"></option></template>
                                    </select>
                                    <select x-model="selectedTargetCommit" :disabled="compareCurrent">
                                        <template x-if="compareCurrent">
                                            <option value="current-codebase">(Current Codebase)</option>
                                        </template>
                                        <template x-for="c in targetCommits" :key="c.hash"><option :value="c.hash" x-text="c.short_hash + ' - ' + c.message.substring(0, 28)"></option></template>
                                    </select>
                                </div>
                            </div>

                            <div>
                                <button class="btn-lime" :disabled="status.is_running" @click="triggerRun()">
                                    <svg class="icon" viewBox="0 0 24 24" style="color: #090a0f;"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg>
                                    <span x-text="status.is_running ? 'RUNNING...' : 'RUN BENCHMARK'"></span>
                                </button>
                            </div>
                        </div>

                        <!-- Live Relay Controls -->
                        <div x-show="currentTargetMode === 'live'" class="controls-row">
                            <div class="control-group" style="grid-column: span 2;">
                                <label>Target Relay URL (ws://...)</label>
                                <input type="text" x-model="liveUrl" placeholder="ws://localhost:7777">
                            </div>
                            <div>
                                <button class="btn-lime" :disabled="status.is_running" @click="triggerRun()">
                                    <svg class="icon" viewBox="0 0 24 24" style="color: #090a0f;"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg>
                                    <span x-text="status.is_running ? 'TESTING...' : 'START LIVE TEST'"></span>
                                </button>
                            </div>
                        </div>

                        <!-- Checkboxes Row -->
                        <div style="display: flex; gap: 20px; margin-top: 14px; font-size: 11px; color: var(--text-secondary); flex-wrap: wrap;">
                            <label class="checkbox-label" x-show="currentTargetMode === 'source'">
                                <input type="checkbox" x-model="compareCurrent"> Use Current Codebase (Working Tree)
                            </label>
                            <label class="checkbox-label" x-show="currentTargetMode === 'source'">
                                <input type="checkbox" x-model="highPerformance"> High-Performance (make -j$(nproc))
                            </label>
                            <label class="checkbox-label">
                                <input type="checkbox" x-model="skipHeavy"> Skip Heavy (1M events)
                            </label>
                            <label class="checkbox-label" x-show="currentTargetMode === 'source'">
                                <input type="checkbox" x-model="fullOutCore"> Out-of-Core Stress (256MB RAM)
                            </label>
                            <label class="checkbox-label" x-show="currentTargetMode === 'source'">
                                <input type="checkbox" x-model="flamegraph"> CPU Flamegraph
                            </label>
                        </div>
                    </div>

                    <!-- Clean Suite List (Self-Expanding Accordions) -->
                    <div style="display: flex; flex-direction: column; gap: 8px;">
                        <template x-for="(s, idx) in suitesConfig" :key="s.id">
                            <div class="suite-card" :class="status.current_suite === s.id ? 'running' : ''">
                                <div class="suite-card-header" @click="toggleSuite(s.id)">
                                    <div style="display: flex; align-items: center; gap: 10px;">
                                        <span x-show="completedMap[s.id]" class="badge completed">✓ COMPLETED</span>
                                        <span x-show="status.current_suite === s.id && !completedMap[s.id]" class="badge running">⚡ TESTING</span>
                                        <span x-show="!completedMap[s.id] && status.current_suite !== s.id" class="badge queued">○ QUEUED</span>
                                        <span class="suite-name" x-text="s.name"></span>
                                    </div>
                                    <div class="suite-metrics-summary">
                                        <template x-if="completedMap[s.id]">
                                            <div style="display: flex; gap: 12px; align-items: center;">
                                                <span style="color: var(--accent-lime); font-weight: 700;" x-text="completedMap[s.id].throughput ? completedMap[s.id].throughput.toFixed(1) + ' ' + (completedMap[s.id].throughput_label || 'ops/s') : ''"></span>
                                                <span x-show="completedMap[s.id].p50_ms" x-text="'P50: ' + (completedMap[s.id].p50_ms ? completedMap[s.id].p50_ms.toFixed(2) + 'ms' : '')"></span>
                                                <span x-show="completedMap[s.id].p99_ms" x-text="'P99: ' + (completedMap[s.id].p99_ms ? completedMap[s.id].p99_ms.toFixed(2) + 'ms' : '')"></span>
                                                <span x-show="completedMap[s.id].memory_rss_mb" style="color: var(--accent-lavender);" x-text="'RSS: ' + (completedMap[s.id].memory_rss_mb ? completedMap[s.id].memory_rss_mb.toFixed(1) + 'MB' : '')"></span>
                                                <span style="color: var(--text-muted);" x-text="completedMap[s.id].elapsed_secs.toFixed(2) + 's'"></span>
                                            </div>
                                        </template>
                                        <template x-if="!completedMap[s.id]">
                                            <span style="color: var(--text-muted);" x-text="status.current_suite === s.id ? 'Testing in progress...' : 'Awaiting runner...'"></span>
                                        </template>
                                        <svg class="icon" viewBox="0 0 24 24" style="color: var(--text-muted);" :style="expandedSuites[s.id] ? 'transform: rotate(180deg);' : ''"><polyline points="6 9 12 15 18 9"/></svg>
                                    </div>
                                </div>

                                <!-- Accordion Body (3-Column Stats Grid + Log) -->
                                <div x-show="expandedSuites[s.id]" class="suite-collapse-body">
                                    <template x-if="completedMap[s.id]">
                                        <div>
                                            <div class="stats-grid">
                                                <div class="stat-panel" x-show="completedMap[s.id].throughput && completedMap[s.id].throughput > 0">
                                                    <div class="stat-panel-title">Throughput & Rate</div>
                                                    <div style="font-family: var(--font-mono); font-size: 11px; line-height: 1.8;">
                                                        <div>Rate: <span style="color: var(--accent-lime); font-weight: 700;" x-text="completedMap[s.id].throughput ? completedMap[s.id].throughput.toFixed(1) + ' ' + (completedMap[s.id].throughput_label || 'ops/s') : '-'"></span></div>
                                                        <template x-for="(v, k) in completedMap[s.id].metrics" :key="k">
                                                            <div x-show="k.includes('tps')" style="color: var(--text-secondary);">
                                                                <span x-text="k.replace(/_/g, ' ') + ':'"></span> <span style="color: var(--accent-lime); font-weight: 600;" x-text="typeof v === 'number' ? v.toFixed(1) : v"></span>
                                                            </div>
                                                        </template>
                                                    </div>
                                                </div>
                                                <div class="stat-panel" x-show="completedMap[s.id].p50_ms && completedMap[s.id].p50_ms > 0">
                                                    <div class="stat-panel-title">Latency Quantiles</div>
                                                    <div style="font-family: var(--font-mono); font-size: 11px; line-height: 1.8;">
                                                        <div x-show="completedMap[s.id].p50_ms">P50: <span style="color: var(--text-primary);" x-text="completedMap[s.id].p50_ms.toFixed(2) + ' ms'"></span></div>
                                                        <div x-show="completedMap[s.id].p90_ms">P90: <span style="color: var(--text-primary);" x-text="completedMap[s.id].p90_ms ? completedMap[s.id].p90_ms.toFixed(2) + ' ms' : ''"></span></div>
                                                        <div x-show="completedMap[s.id].p95_ms">P95: <span style="color: var(--text-primary);" x-text="completedMap[s.id].p95_ms ? completedMap[s.id].p95_ms.toFixed(2) + ' ms' : ''"></span></div>
                                                        <div x-show="completedMap[s.id].p99_ms">P99: <span style="color: var(--text-primary);" x-text="completedMap[s.id].p99_ms ? completedMap[s.id].p99_ms.toFixed(2) + ' ms' : ''"></span></div>
                                                    </div>
                                                </div>
                                                <div class="stat-panel" x-show="getSuiteSpecificMetrics(completedMap[s.id]).length > 0">
                                                    <div class="stat-panel-title">Metrics & Verification</div>
                                                    <div style="font-family: var(--font-mono); font-size: 11px; line-height: 1.8;">
                                                        <template x-for="m in getSuiteSpecificMetrics(completedMap[s.id])" :key="m.key">
                                                            <div>
                                                                <span style="color: var(--text-muted);" x-text="m.key + ':'"></span>
                                                                <span style="color: var(--accent-lime); font-weight: 600; margin-left: 6px;" x-text="m.val"></span>
                                                            </div>
                                                        </template>
                                                    </div>
                                                </div>
                                                <div class="stat-panel" x-show="(completedMap[s.id].elapsed_secs && completedMap[s.id].elapsed_secs > 0) || (completedMap[s.id].memory_rss_mb && completedMap[s.id].memory_rss_mb > 0)">
                                                    <div class="stat-panel-title">Resources & Wall Time</div>
                                                    <div style="font-family: var(--font-mono); font-size: 11px; line-height: 1.8;">
                                                        <div x-show="completedMap[s.id].memory_rss_mb && completedMap[s.id].memory_rss_mb > 0">Memory RSS: <span style="color: var(--accent-lavender);" x-text="completedMap[s.id].memory_rss_mb ? completedMap[s.id].memory_rss_mb.toFixed(1) + ' MB' : '-'"></span></div>
                                                        <div x-show="completedMap[s.id].elapsed_secs && completedMap[s.id].elapsed_secs > 0">Duration: <span style="color: var(--text-primary);" x-text="completedMap[s.id].elapsed_secs.toFixed(2) + ' s'"></span></div>
                                                        <div x-show="completedMap[s.id].metrics && completedMap[s.id].metrics.waf">WAF: <span style="color: var(--text-primary);" x-text="completedMap[s.id].metrics && completedMap[s.id].metrics.waf ? completedMap[s.id].metrics.waf.toFixed(2) + 'x' : ''"></span></div>
                                                    </div>
                                                </div>
                                            </div>

                                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 6px;">
                                                <span style="font-size: 11px; font-weight: 700; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.5px;">Raw Output</span>
                                                <button class="btn-ghost" style="padding: 2px 6px; font-size: 10px;" @click="copyToClipboard(completedMap[s.id].log_output)">
                                                    <svg class="icon" viewBox="0 0 24 24"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>
                                                    Copy
                                                </button>
                                            </div>
                                            <div class="md-rendered-output" x-html="renderMarkdown(completedMap[s.id].log_output)"></div>
                                        </div>
                                    </template>
                                </div>
                            </div>
                        </template>
                    </div>
                </div>

                <!-- 2. COMPARISON VIEW -->
                <div x-show="activeTab === 'comparison'">
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
                        <div class="page-title">
                            <svg class="icon" viewBox="0 0 24 24" style="color: var(--accent-lime);"><line x1="18" x2="18" y1="20" y2="10"/><line x1="12" x2="12" y1="20" y2="4"/><line x1="6" x2="6" y1="20" y2="14"/></svg>
                            <span>A/B Benchmark Comparison</span>
                        </div>
                        <select x-model="selectedComparisonReportId" @change="loadComparisonReport(selectedComparisonReportId)" style="width: 360px;">
                            <template x-for="r in pastComparisonReports" :key="r"><option :value="r" x-text="r"></option></template>
                        </select>
                    </div>

                    <div class="config-card" x-show="comparisonReportData && comparisonReportData.deltas">
                        <table class="diff-table">
                            <thead>
                                <tr>
                                    <th>Metric</th>
                                    <th>Initial</th>
                                    <th>Final</th>
                                    <th>Delta (%)</th>
                                    <th>Status</th>
                                </tr>
                            </thead>
                            <tbody>
                                <template x-for="d in (comparisonReportData ? comparisonReportData.deltas : [])" :key="d.metric">
                                    <tr>
                                        <td style="font-weight: 600;" x-text="d.metric"></td>
                                        <td style="font-family: var(--font-mono);" x-text="d.base_value.toFixed(2) + ' ' + d.unit"></td>
                                        <td style="font-family: var(--font-mono);" x-text="d.target_value.toFixed(2) + ' ' + d.unit"></td>
                                        <td style="font-family: var(--font-mono); font-weight: 700;" x-text="(d.delta_pct > 0 ? '+' : '') + d.delta_pct.toFixed(2) + '%'"></td>
                                        <td><span class="delta-badge" :class="d.status.toLowerCase()" x-text="d.status.toUpperCase()"></span></td>
                                    </tr>
                                </template>
                            </tbody>
                        </table>
                    </div>
                </div>

                <!-- 3. PAST RUNS VIEW (Paired C1 vs C2) -->
                <div x-show="activeTab === 'past'">
                    <div class="config-card" style="margin-bottom: 20px;">
                        <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 14px;">
                            <div>
                                <div class="section-heading" style="margin-bottom: 6px;">Select Historical Benchmark Run</div>
                                <select x-model="selectedPastReportId" @change="loadPastReport(selectedPastReportId)" style="width: 380px;">
                                    <template x-for="r in pastReports" :key="r">
                                        <option :value="r" x-text="r.includes('compare') ? '[A/B COMPARISON] ' + r : '[SINGLE RUN] ' + r"></option>
                                    </template>
                                </select>
                            </div>
                            <div style="display: flex; gap: 6px;">
                                <button class="btn-ghost" :class="pastSubView === 'paired' ? 'active' : ''" @click="pastSubView = 'paired'">
                                    <span x-text="pastReportData && pastReportData.deltas ? 'Paired (C1 vs C2 per Suite)' : '13-Suite Breakdown'"></span>
                                </button>
                                <button class="btn-ghost" :class="pastSubView === 'deltas' ? 'active' : ''" x-show="pastReportData && pastReportData.deltas" @click="pastSubView = 'deltas'">
                                    Deltas Table
                                </button>
                                <button class="btn-ghost" :class="pastSubView === 'markdown' ? 'active' : ''" @click="loadPastMarkdown()">
                                    Markdown
                                </button>
                                <button class="btn-ghost" :class="pastSubView === 'json' ? 'active' : ''" @click="pastSubView = 'json'">
                                    JSON
                                </button>
                            </div>
                        </div>
                    </div>

                    <!-- Paired C1 vs C2 Layout -->
                    <div x-show="pastSubView === 'paired' && pastReportData">
                        <!-- Comparison Runs with deltas -->
                        <div x-show="pastReportData.deltas" style="display: flex; flex-direction: column; gap: 14px;">
                            <template x-for="(s, idx) in (pastReportData.target_report ? pastReportData.target_report.suites : [])" :key="s.id">
                                <div class="suite-card" style="padding: 16px;">
                                    <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px; border-bottom: 1px solid var(--border); padding-bottom: 10px;">
                                        <span style="font-size: 15px; font-weight: 700;" x-text="'#' + (idx + 1) + '. ' + s.name"></span>
                                        <span class="delta-badge" :class="getSuiteStatus(s.id).cls" x-text="getSuiteStatus(s.id).label"></span>
                                    </div>
                                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 14px;">
                                        <!-- Initial C1 -->
                                        <div style="background: #06070b; border: 1px solid rgba(138, 153, 252, 0.35); border-radius: 5px; padding: 12px;">
                                            <div style="display: flex; justify-content: space-between; margin-bottom: 8px;">
                                                <span style="font-family: var(--font-mono); font-size: 11px; color: var(--accent-lavender); font-weight: 700;" x-text="'Initial (C1): ' + pastReportData.base_ref"></span>
                                                <span style="font-family: var(--font-mono); font-size: 11px; color: var(--text-muted);" x-show="getMatchingBaseSuite(s.id)" x-text="getMatchingBaseSuite(s.id) ? getMatchingBaseSuite(s.id).elapsed_secs.toFixed(2) + 's' : ''"></span>
                                            </div>
                                            <div x-show="getMatchingBaseSuite(s.id)">
                                                <div style="font-family: var(--font-mono); font-size: 12px; margin-bottom: 8px;" x-show="getMatchingBaseSuite(s.id) && (getMatchingBaseSuite(s.id).throughput || getMatchingBaseSuite(s.id).p50_ms)">
                                                    <span style="color: var(--accent-lavender); font-weight: 700;" x-show="getMatchingBaseSuite(s.id).throughput" x-text="getMatchingBaseSuite(s.id) && getMatchingBaseSuite(s.id).throughput ? getMatchingBaseSuite(s.id).throughput.toFixed(1) + ' ' + (getMatchingBaseSuite(s.id).throughput_label || 'ops/s') : ''"></span>
                                                    <span style="color: var(--text-secondary); margin-left: 8px;" x-show="getMatchingBaseSuite(s.id).p50_ms" x-text="'P50: ' + (getMatchingBaseSuite(s.id) && getMatchingBaseSuite(s.id).p50_ms ? getMatchingBaseSuite(s.id).p50_ms.toFixed(2) + 'ms' : '')"></span>
                                                </div>
                                                <div style="display: flex; gap: 6px; flex-wrap: wrap; margin-bottom: 8px;" x-show="getMatchingBaseSuite(s.id) && getMatchingBaseSuite(s.id).metrics">
                                                    <template x-for="(v, k) in (getMatchingBaseSuite(s.id) ? getMatchingBaseSuite(s.id).metrics : {})" :key="k">
                                                        <div x-show="typeof v === 'number'" style="background: #0b0d13; border: 1px solid var(--border); padding: 2px 6px; border-radius: 3px; font-family: var(--font-mono); font-size: 10px;">
                                                            <span style="color: var(--text-muted);" x-text="k.replace(/_/g, ' ') + ':'"></span>
                                                            <span style="color: var(--accent-lavender); font-weight: 600;" x-text="typeof v === 'number' ? v.toFixed(1) : v"></span>
                                                        </div>
                                                    </template>
                                                </div>
                                                <div class="md-rendered-output" style="max-height: 250px; overflow-y: auto;" x-html="renderMarkdown(getMatchingBaseSuite(s.id).log_output)"></div>
                                            </div>
                                            <div x-show="!getMatchingBaseSuite(s.id)" style="color: var(--text-muted); font-size: 11px;">Not run on base commit.</div>
                                        </div>

                                        <!-- Final C2 -->
                                        <div style="background: #06070b; border: 1px solid rgba(200, 240, 100, 0.35); border-radius: 5px; padding: 12px;">
                                            <div style="display: flex; justify-content: space-between; margin-bottom: 8px;">
                                                <span style="font-family: var(--font-mono); font-size: 11px; color: var(--accent-lime); font-weight: 700;" x-text="'Final (C2): ' + pastReportData.target_ref"></span>
                                                <span style="font-family: var(--font-mono); font-size: 11px; color: var(--text-muted);" x-text="s.elapsed_secs.toFixed(2) + 's'"></span>
                                            </div>
                                            <div>
                                                <div style="font-family: var(--font-mono); font-size: 12px; margin-bottom: 8px;" x-show="s.throughput || s.p50_ms">
                                                    <span style="color: var(--accent-lime); font-weight: 700;" x-show="s.throughput" x-text="s.throughput ? s.throughput.toFixed(1) + ' ' + (s.throughput_label || 'ops/s') : ''"></span>
                                                    <span style="color: var(--text-secondary); margin-left: 8px;" x-show="s.p50_ms" x-text="'P50: ' + (s.p50_ms ? s.p50_ms.toFixed(2) + 'ms' : '')"></span>
                                                </div>
                                                <div style="display: flex; gap: 6px; flex-wrap: wrap; margin-bottom: 8px;" x-show="s.metrics">
                                                    <template x-for="(v, k) in s.metrics" :key="k">
                                                        <div x-show="typeof v === 'number'" style="background: #0b0d13; border: 1px solid var(--border); padding: 2px 6px; border-radius: 3px; font-family: var(--font-mono); font-size: 10px;">
                                                            <span style="color: var(--text-muted);" x-text="k.replace(/_/g, ' ') + ':'"></span>
                                                            <span style="color: var(--accent-lime); font-weight: 600;" x-text="typeof v === 'number' ? v.toFixed(1) : v"></span>
                                                        </div>
                                                    </template>
                                                </div>
                                                <div class="md-rendered-output" style="max-height: 250px; overflow-y: auto;" x-html="renderMarkdown(s.log_output)"></div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </template>
                        </div>

                        <!-- Single Runs (without deltas) -->
                        <div x-show="!pastReportData.deltas" style="display: flex; flex-direction: column; gap: 14px;">
                            <template x-for="(s, idx) in (pastReportData.suites || [])" :key="s.id">
                                <div class="suite-card" style="padding: 16px;">
                                    <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px; border-bottom: 1px solid var(--border); padding-bottom: 10px;">
                                        <span style="font-size: 15px; font-weight: 700;" x-text="'#' + (idx + 1) + '. ' + s.name"></span>
                                        <span class="delta-badge completed">COMPLETED</span>
                                    </div>
                                    <div style="background: #06070b; border: 1px solid var(--border); border-radius: 5px; padding: 12px;">
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 8px;">
                                            <span style="font-family: var(--font-mono); font-size: 11px; color: var(--accent-lime); font-weight: 700;" x-text="'Commit: ' + (pastReportData.commit ? pastReportData.commit.substring(0, 8) : (pastReportData.git_commit ? pastReportData.git_commit.substring(0, 8) : 'HEAD'))"></span>
                                            <span style="font-family: var(--font-mono); font-size: 11px; color: var(--text-muted);" x-text="s.elapsed_secs.toFixed(2) + 's'"></span>
                                        </div>
                                        <div style="font-family: var(--font-mono); font-size: 12px; margin-bottom: 8px;" x-show="s.throughput || s.p50_ms">
                                            <span style="color: var(--accent-lime); font-weight: 700;" x-show="s.throughput" x-text="s.throughput ? s.throughput.toFixed(1) + ' ' + (s.throughput_label || 'ops/s') : ''"></span>
                                            <span style="color: var(--text-secondary); margin-left: 8px;" x-show="s.p50_ms" x-text="'P50: ' + (s.p50_ms ? s.p50_ms.toFixed(2) + 'ms' : '')"></span>
                                        </div>
                                        <div style="display: flex; gap: 6px; flex-wrap: wrap; margin-bottom: 8px;" x-show="s.metrics">
                                            <template x-for="(v, k) in s.metrics" :key="k">
                                                <div x-show="typeof v === 'number'" style="background: #0b0d13; border: 1px solid var(--border); padding: 2px 6px; border-radius: 3px; font-family: var(--font-mono); font-size: 10px;">
                                                            <span style="color: var(--text-muted);" x-text="k.replace(/_/g, ' ') + ':'"></span>
                                                            <span style="color: var(--accent-lime); font-weight: 600;" x-text="typeof v === 'number' ? v.toFixed(1) : v"></span>
                                                        </div>
                                            </template>
                                        </div>
                                        <div class="md-rendered-output" style="max-height: 250px; overflow-y: auto;" x-html="renderMarkdown(s.log_output)"></div>
                                    </div>
                                </div>
                            </template>
                        </div>
                    </div>

                    <!-- Deltas Table Subview -->
                    <div x-show="pastSubView === 'deltas' && pastReportData && pastReportData.deltas" class="config-card">
                        <table class="diff-table">
                            <thead>
                                <tr><th>Metric</th><th>Initial</th><th>Final</th><th>Delta (%)</th><th>Status</th></tr>
                            </thead>
                            <tbody>
                                <template x-for="d in pastReportData.deltas" :key="d.metric">
                                    <tr>
                                        <td style="font-weight: 600;" x-text="d.metric"></td>
                                        <td style="font-family: var(--font-mono);" x-text="d.base_value.toFixed(2) + ' ' + d.unit"></td>
                                        <td style="font-family: var(--font-mono);" x-text="d.target_value.toFixed(2) + ' ' + d.unit"></td>
                                        <td style="font-family: var(--font-mono); font-weight: 700;" x-text="(d.delta_pct > 0 ? '+' : '') + d.delta_pct.toFixed(2) + '%'"></td>
                                        <td><span class="delta-badge" :class="d.status.toLowerCase()" x-text="d.status.toUpperCase()"></span></td>
                                    </tr>
                                </template>
                            </tbody>
                        </table>
                    </div>

                    <!-- Markdown Subview -->
                    <div x-show="pastSubView === 'markdown'" class="config-card">
                        <div style="display: flex; justify-content: space-between; margin-bottom: 10px;">
                            <div class="section-heading" style="margin-bottom: 0;">Markdown Report</div>
                            <button class="btn-ghost" @click="copyToClipboard(pastMarkdownText)">Copy</button>
                        </div>
                        <pre style="background: #06070a; padding: 14px; border-radius: 5px; border: 1px solid var(--border); font-family: var(--font-mono); font-size: 11px; color: #8c97ad; white-space: pre-wrap; line-height: 1.5;" x-text="pastMarkdownText"></pre>
                    </div>

                    <!-- JSON Subview -->
                    <div x-show="pastSubView === 'json'" class="config-card">
                        <pre style="background: #06070a; padding: 14px; border-radius: 5px; border: 1px solid var(--border); font-family: var(--font-mono); font-size: 11px; color: #8c97ad; white-space: pre-wrap; max-height: 550px; overflow-y: auto;" x-text="JSON.stringify(pastReportData, null, 2)"></pre>
                    </div>
                </div>

                <!-- 4. OVERHAULED ANALYTICS VIEW -->
                <div x-show="activeTab === 'analytics'">
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; flex-wrap: wrap; gap: 14px;">
                        <div class="page-title">
                            <svg class="icon" viewBox="0 0 24 24" style="color: var(--accent-lime); width: 18px; height: 18px;"><line x1="18" x2="18" y1="20" y2="10"/><line x1="12" x2="12" y1="20" y2="4"/><line x1="6" x2="6" y1="20" y2="14"/></svg>
                            <span>Performance Analytics</span>
                        </div>
                        <div style="display: flex; align-items: center; gap: 8px;">
                            <span style="font-size: 11px; font-weight: 700; color: var(--text-secondary); text-transform: uppercase; letter-spacing: 0.5px;">Source:</span>
                            <select x-model="selectedAnalyticsReportId" @change="onAnalyticsSourceChanged()" style="width: 380px;">
                                <option value="live">Live / Current Benchmark Run</option>
                                <template x-for="r in pastReports" :key="r">
                                    <option :value="r" x-text="r.includes('compare') ? '[A/B COMPARISON] ' + r : '[SINGLE RUN] ' + r"></option>
                                </template>
                            </select>
                        </div>
                    </div>

                    <!-- Empty State (When no benchmark data is available) -->
                    <div x-show="getAnalyticsSuites().length === 0" class="config-card" style="text-align: center; padding: 56px 24px;">
                        <div style="font-size: 16px; font-weight: 700; color: var(--text-primary); margin-bottom: 8px;">No Benchmark Data Available</div>
                        <div style="font-size: 13px; color: var(--text-muted); max-width: 480px; margin: 0 auto 20px auto; line-height: 1.5;">
                            Execute a benchmark run from the <strong>Benchmark</strong> tab, or select a historical benchmark run from the dropdown above to view performance analytics.
                        </div>
                        <button class="btn-lime" @click="activeTab = 'runner'" style="display: inline-flex; align-items: center; gap: 6px;">
                            <svg class="icon" viewBox="0 0 24 24" style="color: #090a0f;"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg>
                            Go to Benchmark Runner
                        </button>
                    </div>

                    <!-- Dynamic Analytics Content (When data is available) -->
                    <div x-show="getAnalyticsSuites().length > 0" style="display: flex; flex-direction: column; gap: 16px;">
                        <!-- Latency Quantile Distribution -->
                        <div class="config-card" x-show="getAnalyticsLatencySuites().length > 0">
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
                                <div class="section-heading" style="margin-bottom: 0;">Latency Quantile Distribution (P50 · P90 · P95 · P99)</div>
                                <span style="font-family: var(--font-mono); font-size: 11px; color: var(--text-muted);">Lower is better (ms)</span>
                            </div>
                            <div style="display: flex; flex-direction: column; gap: 14px;">
                                <template x-for="s in getAnalyticsLatencySuites()" :key="s.id">
                                    <div>
                                        <div style="display: flex; justify-content: space-between; font-size: 13px; margin-bottom: 6px;">
                                            <span style="font-weight: 600;" x-text="s.name"></span>
                                            <span style="font-family: var(--font-mono); font-size: 11px; color: var(--text-secondary);">
                                                <span x-show="s.p50_ms" x-text="'P50: ' + s.p50_ms.toFixed(2) + 'ms'"></span>
                                                <span x-show="s.p90_ms" x-text="' | P90: ' + s.p90_ms.toFixed(2) + 'ms'"></span>
                                                <span x-show="s.p95_ms" x-text="' | P95: ' + s.p95_ms.toFixed(2) + 'ms'"></span>
                                                <span x-show="s.p99_ms" x-text="' | P99: ' + s.p99_ms.toFixed(2) + 'ms'"></span>
                                            </span>
                                        </div>
                                        <div class="quantile-bar">
                                            <div :style="'width: ' + getQuantileWidth(s.p50_ms, getMaxLatency()) + '%; background: var(--accent-lime);'" :title="'P50: ' + (s.p50_ms ? s.p50_ms.toFixed(2) : '') + 'ms'"></div>
                                            <div :style="'width: ' + getQuantileWidth((s.p90_ms || s.p50_ms) - s.p50_ms, getMaxLatency()) + '%; background: #a4db4a;'" :title="'P90: ' + (s.p90_ms ? s.p90_ms.toFixed(2) : '') + 'ms'"></div>
                                            <div :style="'width: ' + getQuantileWidth((s.p95_ms || s.p90_ms || s.p50_ms) - (s.p90_ms || s.p50_ms), getMaxLatency()) + '%; background: #e0b438;'" :title="'P95: ' + (s.p95_ms ? s.p95_ms.toFixed(2) : '') + 'ms'"></div>
                                            <div :style="'width: ' + getQuantileWidth((s.p99_ms || s.p95_ms || s.p50_ms) - (s.p95_ms || s.p90_ms || s.p50_ms), getMaxLatency()) + '%; background: var(--accent-red);'" :title="'P99: ' + (s.p99_ms ? s.p99_ms.toFixed(2) : '') + 'ms'"></div>
                                        </div>
                                    </div>
                                </template>
                            </div>
                        </div>

                        <!-- Throughput & Memory Scaling Grid -->
                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px;">
                            <div class="config-card" x-show="getAnalyticsThroughputSuites().length > 0">
                                <div class="section-heading">Peak Throughput Ranking</div>
                                <div style="display: flex; flex-direction: column; gap: 12px; margin-top: 14px;">
                                    <template x-for="s in getAnalyticsThroughputSuites()" :key="s.id">
                                        <div class="bar-chart-row">
                                            <div class="bar-chart-label">
                                                <span x-text="s.name"></span>
                                                <span style="font-family: var(--font-mono); color: var(--accent-lime); font-weight: 700;" x-text="s.throughput ? s.throughput.toFixed(1) + ' ' + (s.throughput_label || 'ops/s') : '-'"></span>
                                            </div>
                                            <div class="bar-chart-track">
                                                <div class="bar-chart-fill" :style="'width: ' + Math.max(3, (s.throughput / getMaxThroughput()) * 100) + '%;'"></div>
                                            </div>
                                        </div>
                                    </template>
                                </div>
                            </div>

                            <div class="config-card" x-show="getAnalyticsMemorySuites().length > 0">
                                <div class="section-heading">Process Memory Usage (VmRSS per Suite)</div>
                                <div style="display: flex; flex-direction: column; gap: 12px; margin-top: 14px;">
                                    <template x-for="s in getAnalyticsMemorySuites()" :key="s.id">
                                        <div class="bar-chart-row">
                                            <div class="bar-chart-label">
                                                <span x-text="s.name"></span>
                                                <span style="font-family: var(--font-mono); color: var(--accent-lavender); font-weight: 700;" x-text="s.memory_rss_mb ? s.memory_rss_mb.toFixed(1) + ' MB' : '-'"></span>
                                            </div>
                                            <div class="bar-chart-track">
                                                <div class="bar-chart-fill" :style="'width: ' + Math.max(3, (s.memory_rss_mb / getMaxMemory()) * 100) + '%; background: var(--accent-lavender);'"></div>
                                            </div>
                                        </div>
                                    </template>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- 5. FLAMEGRAPH VIEW -->
                <div x-show="activeTab === 'flamegraph'">
                    <div class="flamegraph-viewer">
                        <div class="flamegraph-toolbar">
                            <div style="display: flex; align-items: center; gap: 10px;">
                                <span style="font-size: 11px; font-weight: 700; color: var(--text-secondary); text-transform: uppercase; letter-spacing: 0.5px;">Run</span>
                                <select x-model="selectedFlamegraphReportId" @change="onFlamegraphReportChanged()" style="width: 300px; font-size: 11px;">
                                    <template x-for="r in pastReports" :key="r"><option :value="r" x-text="r"></option></template>
                                </select>
                                <span style="font-size: 11px; font-weight: 700; color: var(--text-secondary); text-transform: uppercase; letter-spacing: 0.5px;">Profile</span>
                                <select x-model="selectedFlamegraphFile" @change="onFlamegraphFileChanged()" style="width: 220px; font-size: 11px;">
                                    <template x-for="f in flamegraphFiles" :key="f.file"><option :value="f.file" x-text="f.label"></option></template>
                                </select>
                            </div>
                            <div style="display: flex; gap: 6px;">
                                <button class="btn-ghost" @click="refreshFlamegraph()">Refresh</button>
                                <a x-show="selectedFlamegraphFile" class="btn-ghost" style="text-decoration: none;" :href="flamegraphUrl" target="_blank" download>Download SVG</a>
                            </div>
                        </div>
                        <iframe x-show="flamegraphFiles.length > 0" class="flamegraph-frame" :src="flamegraphUrl" scrolling="no" @load="fitFlamegraph($event.target)"></iframe>
                        <div x-show="flamegraphFiles.length === 0" style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 500px; text-align: center; color: var(--text-muted); font-size: 13px;">
                            No flamegraph SVG found for this run. Run with "CPU Flamegraph" enabled.
                        </div>
                    </div>
                </div>
            </main>

            <!-- Sidebar Telemetry -->
            <aside x-show="activeTab === 'runner'">
                <div class="sidebar-card">
                    <div class="section-heading" style="color: var(--accent-cyan); margin-bottom: 8px;">Live Telemetry & Logs</div>
                    <div style="font-size: 12px; font-weight: 600; color: var(--accent-cyan); margin-bottom: 8px;" x-text="status.current_step"></div>
                    <div class="log-terminal" id="log-terminal">
                        <template x-for="(line, i) in logs" :key="i">
                            <div x-text="line"></div>
                        </template>
                    </div>
                </div>
            </aside>
        </div>

        <footer class="footer">
            <div>
                <span class="brand-badge" style="font-size: 10px; padding: 2px 6px;">STRFRY</span>
                <span x-text="status.is_running ? 'Benchmark Running...' : 'Ready'"></span>
            </div>
            <div>~/strfry-bench/report</div>
        </footer>
    </div>

    <script>
        function benchApp() {
            return {
                activeTab: 'runner',
                currentTargetMode: 'source',
                currentTestMode: 'single',
                compareCurrent: false,
                highPerformance: false,
                skipHeavy: true,
                fullOutCore: false,
                flamegraph: true,
                liveUrl: 'ws://localhost:7777',

                branches: ['master', 'feat/benchmarking'],
                selectedBaseBranch: 'master',
                selectedTargetBranch: 'feat/benchmarking',
                baseCommits: [],
                targetCommits: [],
                selectedBaseCommit: 'HEAD',
                selectedTargetCommit: 'HEAD',

                status: {
                    is_running: false,
                    current_suite: null,
                    current_step: 'Idle · Ready to benchmark',
                    total_suites: 13,
                    completed_suites_count: 0,
                    peak_tps: 0,
                    best_p99_ms: null,
                    peak_rss_mb: 0,
                    elapsed_secs: 0
                },

                suitesConfig: [
                    { id: "storage", name: "Storage (In-Core vs Out-of-Core)" },
                    { id: "ingestion", name: "Event Ingestion Pipeline" },
                    { id: "concurrency", name: "Concurrency & Thread Pool" },
                    { id: "websockets", name: "WebSockets & Connections" },
                    { id: "queries", name: "Query Engine & Indices" },
                    { id: "monitors", name: "Active Monitors (Fanout)" },
                    { id: "negentropy", name: "Negentropy Sync" },
                    { id: "plugin", name: "Write Policy Plugin" },
                    { id: "cli", name: "CLI & Dictionary Compression" },
                    { id: "os", name: "OS-Level Metrics & WAF" },
                    { id: "stress", name: "Stress & Adversarial Attacks" },
                    { id: "backpressure", name: "Backpressure Performance" },
                    { id: "deterministic", name: "Deterministic Instructions & Allocations" }
                ],

                completedMap: {},
                expandedSuites: {},

                pastReports: [],
                pastComparisonReports: [],
                selectedComparisonReportId: '',
                comparisonReportData: null,

                selectedPastReportId: '',
                pastReportData: null,
                pastSubView: 'paired',
                pastMarkdownText: '',

                selectedAnalyticsReportId: 'live',
                analyticsReportData: null,
                selectedFlamegraphReportId: '',
                flamegraphFiles: [],
                selectedFlamegraphFile: '',
                flamegraphUrl: 'about:blank',

                logs: ['Connecting to live benchmark stream...'],
                wsConnected: false,

                toggleSuite(id) {
                    this.expandedSuites[id] = !this.expandedSuites[id];
                },

                getMatchingBaseSuite(id) {
                    if (!this.pastReportData || !this.pastReportData.base_report) return null;
                    return this.pastReportData.base_report.suites.find(s => s.id === id);
                },

                getSuiteStatus(suiteId) {
                    if (!this.pastReportData) return { label: 'COMPLETED', cls: 'stable' };
                    if (this.pastReportData.deltas && this.pastReportData.deltas.length > 0) {
                        const sid = suiteId.toLowerCase();
                        const matching = this.pastReportData.deltas.filter(d => {
                            const m = (d.metric || '').toLowerCase();
                            return m.includes(sid) ||
                                (sid === 'storage' && (m.includes('scan') || m.includes('pagination') || m.includes('in_core'))) ||
                                (sid === 'ingestion' && m.includes('event')) ||
                                (sid === 'concurrency' && (m.includes('concurrency') || m.includes('churn'))) ||
                                (sid === 'connections' && m.includes('connection')) ||
                                (sid === 'req' && m.includes('req')) ||
                                (sid === 'monitor' && m.includes('monitor')) ||
                                (sid === 'negentropy' && m.includes('negentropy')) ||
                                (sid === 'plugin' && m.includes('plugin')) ||
                                (sid === 'os_stress' && m.includes('stress')) ||
                                (sid === 'cli_dict' && (m.includes('dict') || m.includes('import'))) ||
                                (sid === 'malicious' && m.includes('malicious')) ||
                                (sid === 'backpressure' && m.includes('backpressure')) ||
                                (sid === 'churn' && m.includes('churn'));
                        });

                        if (matching.length > 0) {
                            const hasRegressed = matching.some(d => (d.status || '').toLowerCase() === 'regressed');
                            const hasImproved = matching.some(d => (d.status || '').toLowerCase() === 'improved');
                            if (hasRegressed) return { label: 'REGRESSED', cls: 'regressed' };
                            if (hasImproved) return { label: 'IMPROVED', cls: 'improved' };
                            return { label: 'STABLE', cls: 'stable' };
                        }

                        // Fallback: compare target suite vs base suite throughput & elapsed time
                        const targetSuite = (this.pastReportData.target_report ? this.pastReportData.target_report.suites : []).find(s => s.id === suiteId);
                        const baseSuite = this.getMatchingBaseSuite(suiteId);
                        if (targetSuite && baseSuite) {
                            if (targetSuite.throughput && baseSuite.throughput) {
                                const diff = ((targetSuite.throughput - baseSuite.throughput) / baseSuite.throughput) * 100;
                                if (diff > 1.0) return { label: 'IMPROVED', cls: 'improved' };
                                if (diff < -1.0) return { label: 'REGRESSED', cls: 'regressed' };
                            } else if (targetSuite.elapsed_secs && baseSuite.elapsed_secs) {
                                const diff = ((targetSuite.elapsed_secs - baseSuite.elapsed_secs) / baseSuite.elapsed_secs) * 100;
                                if (diff < -1.0) return { label: 'IMPROVED', cls: 'improved' };
                                if (diff > 1.0) return { label: 'REGRESSED', cls: 'regressed' };
                            }
                            return { label: 'STABLE', cls: 'stable' };
                        }
                    }
                    return { label: 'COMPLETED', cls: 'completed' };
                },

                fitFlamegraph(iframe) {
                    try {
                        if (iframe && iframe.contentWindow && iframe.contentWindow.document) {
                            const doc = iframe.contentWindow.document;
                            const svg = doc.querySelector('svg');
                            if (svg) {
                                const hAttr = svg.getAttribute('height');
                                const docH = doc.documentElement.scrollHeight || (doc.body ? doc.body.scrollHeight : 0);
                                const targetH = Math.max(parseInt(hAttr) || 0, docH || 0, 700);
                                iframe.style.height = (targetH + 20) + 'px';
                            }
                        }
                    } catch (e) {}
                },

                renderMarkdown(text) {
                    if (!text) return '';
                    let escaped = text
                        .replace(/&/g, '&amp;')
                        .replace(/</g, '&lt;')
                        .replace(/>/g, '&gt;');

                    escaped = escaped.replace(/```(?:[a-zA-Z]*)\n?([\s\S]*?)```/g, (match, code) => {
                        return `<pre class="md-code-block">${code.trim()}</pre>`;
                    });

                    escaped = escaped.replace(/^###\s+(.*?)$/gm, '<div class="md-heading">$1</div>');
                    escaped = escaped.replace(/^##\s+(.*?)$/gm, '<div class="md-heading" style="font-size: 13px;">$1</div>');

                    escaped = escaped.replace(/^[-*]\s+\*\*(.*?)(?::\*\*|\*\*[:]*)\s*(.*?)$/gm, (match, key, val) => {
                        return `<div class="md-kv-row"><span class="md-key">${key.trim()}:</span> <span class="md-val">${val.trim()}</span></div>`;
                    });

                    escaped = escaped.replace(/\*\*(.*?)\*\*/g, '<strong style="color: var(--text-primary); font-weight: 700;">$1</strong>');

                    escaped = escaped.replace(/^[-*]\s+(.*?)$/gm, '<div class="md-bullet-item"><span class="md-bullet">•</span> $1</div>');

                    return escaped;
                },

                getSuiteSpecificMetrics(suite) {
                    if (!suite || !suite.metrics || typeof suite.metrics !== 'object') return [];
                    const res = [];
                    for (const [k, v] of Object.entries(suite.metrics)) {
                        if (k === 'process_rss_mb' || k === 'peak_rss_mb' || k === 'cpu_percent' || 
                            k === 'avg_cpu_percent' || k.includes('tps') || k === 'waf' || 
                            k === 'info' || k.includes('output') || k === 'mdb_stat' || 
                            k === 'connection_memory' || typeof v === 'object' || typeof v === 'boolean') {
                            continue;
                        }
                        if (typeof v === 'string' && (v.includes('\n') || v.length > 50)) {
                            continue;
                        }

                        let valStr = v;
                        if (typeof v === 'number') {
                            if (k.includes('_time') || k.includes('time_sec')) {
                                valStr = v.toFixed(3) + ' s';
                            } else if (k.includes('_mb')) {
                                valStr = v.toFixed(2) + ' MB';
                            } else if (Number.isInteger(v)) {
                                valStr = v.toLocaleString();
                            } else {
                                valStr = v.toFixed(2);
                            }
                        }
                        res.push({ key: k.replace(/_/g, ' '), val: valStr });
                    }
                    return res;
                },
                async fetchBranches() {
                    try {
                        const res = await fetch('/api/branches');
                        const data = await res.json();
                        if (Array.isArray(data) && data.length > 0) {
                            this.branches = data;
                            if (this.branches.includes('master')) this.selectedBaseBranch = 'master';
                            if (this.branches.includes('feat/benchmarking')) this.selectedTargetBranch = 'feat/benchmarking';
                        }
                        await this.onBaseBranchChanged();
                        await this.onTargetBranchChanged();
                    } catch (e) {}
                },

                async onBaseBranchChanged() {
                    try {
                        const res = await fetch(`/api/commits?branch=${encodeURIComponent(this.selectedBaseBranch)}`);
                        this.baseCommits = await res.json();
                        if (this.baseCommits.length > 0) {
                            this.selectedBaseCommit = this.baseCommits[0].hash;
                        }
                    } catch (e) {}
                },

                async onTargetBranchChanged() {
                    try {
                        const res = await fetch(`/api/commits?branch=${encodeURIComponent(this.selectedTargetBranch)}`);
                        this.targetCommits = await res.json();
                        if (this.targetCommits.length > 0) {
                            this.selectedTargetCommit = this.targetCommits[0].hash;
                        }
                    } catch (e) {}
                },

                async triggerRun() {
                    this.status.is_running = true;
                    this.status.current_step = 'Initiating build and test execution...';
                    this.status.current_suite = 'Building strfry...';
                    this.status.completed_suites_count = 0;
                    this.status.elapsed_secs = 0;
                    this.completedMap = {};
                    this.expandedSuites = {};
                    this.logs = ['[BENCH] Triggering benchmark run from Web UI...'];

                    const isCurrent = this.compareCurrent && this.currentTargetMode === 'source';
                    const targetCommitVal = isCurrent ? "current-codebase" : this.selectedTargetCommit;

                    const req = {
                        target_mode: this.currentTargetMode,
                        url: this.liveUrl,
                        test_type: this.currentTestMode,
                        base: this.selectedBaseCommit,
                        target: targetCommitVal,
                        current: isCurrent,
                        high_performance: this.highPerformance,
                        skip_heavy: this.skipHeavy,
                        full: this.fullOutCore,
                        flamegraph: this.flamegraph
                    };

                    try {
                        await fetch('/api/run', {
                            method: 'POST',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify(req)
                        });
                    } catch (e) {
                        this.status.current_step = 'Failed to trigger run: ' + e;
                    }
                },

                async refreshStatus() {
                    try {
                        const res = await fetch('/api/status');
                        const data = await res.json();
                        this.updateTelemetry(data.status, data.completed_suites);
                    } catch (e) {}
                },

                updateTelemetry(status, completedSuites) {
                    this.status = status;
                    if (completedSuites) {
                        completedSuites.forEach(s => {
                            if (!this.completedMap[s.id]) {
                                this.expandedSuites[s.id] = true;
                            }
                            this.completedMap[s.id] = s;
                        });
                    }
                },

                async openComparisonTab() {
                    this.activeTab = 'comparison';
                    await this.loadReportsList();
                    if (this.pastComparisonReports.length > 0 && !this.selectedComparisonReportId) {
                        this.selectedComparisonReportId = this.pastComparisonReports[0];
                        await this.loadComparisonReport(this.selectedComparisonReportId);
                    }
                },

                async loadComparisonReport(reportId) {
                    if (!reportId) return;
                    try {
                        const res = await fetch(`/api/reports/${encodeURIComponent(reportId)}`);
                        this.comparisonReportData = await res.json();
                    } catch (e) {}
                },

                async openPastTab() {
                    this.activeTab = 'past';
                    await this.loadReportsList();
                    if (this.pastReports.length > 0 && !this.selectedPastReportId) {
                        this.selectedPastReportId = this.pastReports[0];
                        await this.loadPastReport(this.selectedPastReportId);
                    }
                },

                async openAnalyticsTab() {
                    this.activeTab = 'analytics';
                    await this.loadReportsList();
                    if (this.selectedAnalyticsReportId !== 'live') {
                        await this.onAnalyticsSourceChanged();
                    }
                },

                async onAnalyticsSourceChanged() {
                    if (this.selectedAnalyticsReportId === 'live') {
                        this.analyticsReportData = null;
                    } else {
                        try {
                            const res = await fetch(`/api/reports/${encodeURIComponent(this.selectedAnalyticsReportId)}`);
                            this.analyticsReportData = await res.json();
                        } catch (e) {
                            this.analyticsReportData = null;
                        }
                    }
                },

                getAnalyticsSuites() {
                    if (this.selectedAnalyticsReportId !== 'live' && this.analyticsReportData) {
                        if (this.analyticsReportData.target_report && this.analyticsReportData.target_report.suites) {
                            return this.analyticsReportData.target_report.suites;
                        }
                        if (this.analyticsReportData.suites) {
                            return this.analyticsReportData.suites;
                        }
                    }
                    return Object.values(this.completedMap);
                },

                getAnalyticsLatencySuites() {
                    return this.getAnalyticsSuites().filter(s => s && (s.p50_ms || s.p90_ms || s.p99_ms));
                },

                getMaxLatency() {
                    const lats = this.getAnalyticsLatencySuites().map(s => s.p99_ms || s.p95_ms || s.p90_ms || s.p50_ms || 0);
                    return Math.max(...lats, 10);
                },

                getQuantileWidth(val, maxVal) {
                    if (!val || !maxVal || maxVal <= 0) return 0;
                    return Math.min(100, Math.max(1, (val / maxVal) * 100));
                },

                getAnalyticsThroughputSuites() {
                    return this.getAnalyticsSuites()
                        .filter(s => s && s.throughput && s.throughput > 0)
                        .sort((a, b) => (b.throughput || 0) - (a.throughput || 0));
                },

                getMaxThroughput() {
                    const tps = this.getAnalyticsThroughputSuites().map(s => s.throughput || 0);
                    return Math.max(...tps, 1);
                },

                getAnalyticsMemorySuites() {
                    return this.getAnalyticsSuites()
                        .filter(s => s && s.memory_rss_mb && s.memory_rss_mb > 0)
                        .sort((a, b) => (b.memory_rss_mb || 0) - (a.memory_rss_mb || 0));
                },

                getMaxMemory() {
                    const mems = this.getAnalyticsMemorySuites().map(s => s.memory_rss_mb || 0);
                    return Math.max(...mems, 1);
                },

                async loadReportsList() {
                    try {
                        const res = await fetch('/api/reports');
                        const reports = await res.json();
                        this.pastReports = reports;
                        this.pastComparisonReports = reports.filter(r => r.includes('compare'));
                    } catch (e) {}
                },

                async loadPastReport(reportId) {
                    if (!reportId) return;
                    try {
                        const res = await fetch(`/api/reports/${encodeURIComponent(reportId)}`);
                        this.pastReportData = await res.json();
                        this.pastSubView = 'paired';
                    } catch (e) {}
                },

                async loadPastMarkdown() {
                    this.pastSubView = 'markdown';
                    if (!this.selectedPastReportId || !this.pastReportData) return;
                    try {
                        const fileName = this.pastReportData.deltas ? 'comparison.md' : 'summary.md';
                        const res = await fetch(`/api/reports/${encodeURIComponent(this.selectedPastReportId)}/${fileName}`);
                        this.pastMarkdownText = await res.text();
                    } catch (e) {
                        this.pastMarkdownText = 'Failed to load markdown: ' + e;
                    }
                },

                async openFlamegraphTab() {
                    this.activeTab = 'flamegraph';
                    await this.loadReportsList();
                    if (this.pastReports.length > 0 && !this.selectedFlamegraphReportId) {
                        this.selectedFlamegraphReportId = this.pastReports[0];
                        await this.onFlamegraphReportChanged();
                    }
                },

                async onFlamegraphReportChanged() {
                    if (!this.selectedFlamegraphReportId) return;
                    try {
                        const res = await fetch(`/api/reports/${encodeURIComponent(this.selectedFlamegraphReportId)}/files`);
                        const files = await res.json();
                        const svgFiles = files.filter(f => f.endsWith('.svg'));
                        this.flamegraphFiles = [];

                        svgFiles.forEach(f => {
                            let label = f;
                            if (f === 'flamegraph.svg') label = 'Flamegraph (Full Run)';
                            else if (f === 'final_flamegraph.svg') label = 'Final Flamegraph (Target)';
                            else if (f === 'initial_flamegraph.svg') label = 'Initial Flamegraph (Base)';
                            this.flamegraphFiles.push({ file: f, label });
                        });

                        if (this.flamegraphFiles.length > 0) {
                            this.selectedFlamegraphFile = this.flamegraphFiles[0].file;
                        } else {
                            this.selectedFlamegraphFile = '';
                        }
                        this.onFlamegraphFileChanged();
                    } catch (e) {}
                },

                onFlamegraphFileChanged() {
                    if (!this.selectedFlamegraphReportId || !this.selectedFlamegraphFile) {
                        this.flamegraphUrl = 'about:blank';
                    } else {
                        this.flamegraphUrl = `/api/reports/${encodeURIComponent(this.selectedFlamegraphReportId)}/${encodeURIComponent(this.selectedFlamegraphFile)}`;
                        setTimeout(() => {
                            const iframe = document.querySelector('.flamegraph-frame');
                            if (iframe) this.fitFlamegraph(iframe);
                        }, 150);
                    }
                },

                async refreshFlamegraph() {
                    await this.loadReportsList();
                    await this.onFlamegraphReportChanged();
                },

                copyToClipboard(text) {
                    navigator.clipboard.writeText(text);
                },

                ws: null,

                connectWs() {
                    if (this.ws && (this.ws.readyState === WebSocket.OPEN || this.ws.readyState === WebSocket.CONNECTING)) {
                        return;
                    }

                    const protocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
                    const ws = new WebSocket(`${protocol}//${location.host}/api/ws`);
                    this.ws = ws;

                    ws.onopen = () => { this.wsConnected = true; };

                    ws.onmessage = (event) => {
                        const data = JSON.parse(event.data);
                        if (data.type === 'telemetry') {
                            this.updateTelemetry(data.status, data.completed_suites);
                        } else if (data.type === 'log') {
                            this.logs.push(data.line);
                            setTimeout(() => {
                                const term = document.getElementById('log-terminal');
                                if (term) term.scrollTop = term.scrollHeight;
                            }, 40);
                        }
                    };

                    ws.onclose = () => {
                        this.wsConnected = false;
                        this.ws = null;
                        setTimeout(() => this.connectWs(), 2000);
                    };
                },
                init() {
                    this.fetchBranches();
                    this.loadReportsList();
                    this.refreshStatus();
                    this.connectWs();
                }
            };
        }
    </script>
</body>
</html>
"##;
