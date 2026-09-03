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
            --bg-canvas: #0b0e17;
            --bg-card: #131929;
            --bg-card-hover: #161d31;
            --border: #1e2740;
            --border-highlight: #2c3859;
            --text-primary: #e6edf3;
            --text-secondary: #8b9bb4;
            --text-muted: #566583;
            --accent-lime: #d2f884;
            --accent-lime-bg: #1c2712;
            --accent-cyan: #56d4f5;
            --accent-cyan-bg: #102636;
            --accent-lavender: #8a99fc;
            --accent-lavender-bg: #191c33;
            --accent-red: #f87171;
            --accent-red-bg: #2b1414;
            --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            --font-mono: "JetBrains Mono", "Fira Code", monospace;
        }

        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        ::-webkit-scrollbar {
            width: 5px;
            height: 5px;
        }
        ::-webkit-scrollbar-track {
            background: var(--bg-canvas);
        }
        ::-webkit-scrollbar-thumb {
            background: var(--border-highlight);
            border-radius: 2px;
        }
        ::-webkit-scrollbar-thumb:hover {
            background: var(--text-muted);
        }

        body {
            background-color: var(--bg-canvas);
            color: var(--text-primary);
            font-family: var(--font-sans);
            line-height: 1.5;
            overflow-x: hidden;
            display: flex;
            flex-direction: column;
            min-height: 100vh;
        }

        /* Top Navigation Bar */
        .navbar {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 12px 32px;
            background-color: var(--bg-canvas);
            border-bottom: 1px solid var(--border);
            position: sticky;
            top: 0;
            z-index: 100;
        }

        .nav-left {
            display: flex;
            align-items: center;
            gap: 24px;
        }

        .brand-badge {
            background-color: var(--accent-lime);
            color: #0b0e17;
            font-weight: 800;
            font-size: 13px;
            padding: 4px 10px;
            border-radius: 4px;
            letter-spacing: 0.5px;
            text-transform: uppercase;
        }

        .nav-links {
            display: flex;
            gap: 20px;
        }

        .nav-link {
            color: var(--text-secondary);
            font-size: 13px;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.8px;
            cursor: pointer;
            transition: color 0.15s ease;
            text-decoration: none;
            padding: 4px 0;
            user-select: none;
        }

        .nav-link:hover, .nav-link.active {
            color: var(--text-primary);
            border-bottom: 2px solid var(--accent-lime);
        }

        .nav-right {
            display: flex;
            align-items: center;
            gap: 12px;
        }

        .btn-ghost {
            background: transparent;
            border: 1px solid var(--border);
            color: var(--text-secondary);
            font-size: 12px;
            padding: 6px 12px;
            border-radius: 6px;
            cursor: pointer;
            display: flex;
            align-items: center;
            gap: 6px;
            transition: all 0.15s ease;
            user-select: none;
        }

        .btn-ghost:hover, .btn-ghost.active {
            background-color: var(--bg-card);
            color: var(--text-primary);
            border-color: var(--border-highlight);
        }

        .status-dot {
            width: 8px;
            height: 8px;
            border-radius: 50%;
            background-color: var(--accent-lime);
            box-shadow: 0 0 8px var(--accent-lime);
        }

        .status-dot.disconnected {
            background-color: var(--accent-red);
            box-shadow: 0 0 8px var(--accent-red);
        }

        /* Container Layout */
        .container {
            max-width: 1560px;
            margin: 0 auto;
            padding: 24px 32px;
            width: 100%;
            display: grid;
            grid-template-columns: 1fr 380px;
            gap: 32px;
            flex: 1;
        }

        @media (max-width: 1200px) {
            .container {
                grid-template-columns: 1fr;
            }
        }

        .section-comment {
            font-family: var(--font-mono);
            font-size: 12px;
            color: var(--text-muted);
            margin-bottom: 8px;
            letter-spacing: 0.2px;
        }

        .page-title {
            font-size: 26px;
            font-weight: 700;
            letter-spacing: -0.5px;
            margin-bottom: 20px;
            display: flex;
            align-items: center;
            gap: 12px;
        }

        /* KPI 6-Card Grid */
        .kpi-grid {
            display: grid;
            grid-template-columns: repeat(6, 1fr);
            gap: 14px;
            margin-bottom: 28px;
        }

        @media (max-width: 1100px) {
            .kpi-grid {
                grid-template-columns: repeat(3, 1fr);
            }
        }

        @media (max-width: 650px) {
            .kpi-grid {
                grid-template-columns: repeat(2, 1fr);
            }
        }

        .kpi-card {
            background-color: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 14px 16px;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            min-height: 98px;
            transition: border-color 0.15s ease;
        }

        .kpi-card:hover {
            border-color: var(--border-highlight);
        }

        .kpi-header {
            display: flex;
            align-items: center;
            gap: 6px;
            font-size: 11px;
            font-weight: 600;
            color: var(--text-secondary);
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        .kpi-value {
            font-size: 22px;
            font-weight: 700;
            font-family: var(--font-mono);
            color: var(--text-primary);
            margin: 4px 0;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }

        .kpi-footer {
            font-size: 10px;
            color: var(--text-muted);
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        /* Progress Bar */
        .progress-section {
            margin-bottom: 28px;
        }

        .progress-bar-bg {
            width: 100%;
            height: 6px;
            background-color: var(--bg-card);
            border-radius: 3px;
            overflow: hidden;
            margin-top: 8px;
            border: 1px solid var(--border);
        }

        .progress-bar-fill {
            height: 100%;
            background-color: var(--accent-lime);
            transition: width 0.3s ease;
            box-shadow: 0 0 10px rgba(210, 248, 132, 0.4);
        }

        /* Action & Controls Card */
        .config-card {
            background-color: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 20px;
            margin-bottom: 28px;
        }

        .config-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 16px;
            flex-wrap: wrap;
            gap: 12px;
        }

        .mode-toggle {
            display: flex;
            background-color: var(--bg-canvas);
            border: 1px solid var(--border);
            border-radius: 6px;
            padding: 3px;
            gap: 4px;
        }

        .mode-btn {
            background: transparent;
            border: none;
            color: var(--text-secondary);
            font-size: 12px;
            font-weight: 600;
            padding: 6px 14px;
            border-radius: 4px;
            cursor: pointer;
            transition: all 0.15s ease;
            user-select: none;
        }

        .mode-btn.active {
            background-color: var(--bg-card-hover);
            color: var(--accent-lime);
        }

        .controls-row {
            display: grid;
            grid-template-columns: 1fr 1fr auto;
            gap: 16px;
            align-items: end;
        }

        @media (max-width: 900px) {
            .controls-row {
                grid-template-columns: 1fr;
            }
        }

        .control-group label {
            display: block;
            font-size: 11px;
            text-transform: uppercase;
            color: var(--text-secondary);
            margin-bottom: 6px;
            font-weight: 600;
        }

        select, input[type="text"] {
            width: 100%;
            background-color: var(--bg-canvas);
            border: 1px solid var(--border);
            color: var(--text-primary);
            padding: 8px 12px;
            border-radius: 6px;
            font-size: 13px;
            outline: none;
            transition: border-color 0.15s ease;
        }

        select:focus, input[type="text"]:focus {
            border-color: var(--accent-lime);
        }

        select:disabled, input[type="text"]:disabled {
            opacity: 0.5;
            cursor: not-allowed;
        }

        .btn-lime {
            background-color: var(--accent-lime);
            color: #0b0e17;
            font-weight: 700;
            font-size: 13px;
            padding: 10px 22px;
            border: none;
            border-radius: 6px;
            cursor: pointer;
            transition: opacity 0.15s ease;
            display: flex;
            align-items: center;
            gap: 8px;
            white-space: nowrap;
            user-select: none;
        }

        .btn-lime:hover {
            opacity: 0.9;
        }

        .btn-lime:disabled {
            opacity: 0.5;
            cursor: not-allowed;
        }

        .checkbox-label {
            display: flex;
            align-items: center;
            gap: 8px;
            font-size: 12px;
            color: var(--text-secondary);
            cursor: pointer;
            margin-top: 12px;
            user-select: none;
        }

        /* Suite Collapsible Card */
        .suite-card {
            background-color: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: 8px;
            overflow: hidden;
            margin-bottom: 12px;
            transition: border-color 0.15s ease;
        }

        .suite-card.running {
            border-color: var(--accent-cyan);
            box-shadow: 0 0 12px rgba(86, 212, 245, 0.15);
        }

        .suite-card-header {
            padding: 14px 20px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            cursor: pointer;
            user-select: none;
            transition: background 0.15s ease;
        }

        .suite-card-header:hover {
            background-color: var(--bg-card-hover);
        }

        .suite-info {
            display: flex;
            align-items: center;
            gap: 14px;
        }

        .badge-status {
            font-size: 11px;
            font-weight: 700;
            font-family: var(--font-mono);
            padding: 3px 8px;
            border-radius: 4px;
            text-transform: uppercase;
        }

        .badge-status.completed {
            background-color: var(--accent-lime-bg);
            color: var(--accent-lime);
            border: 1px solid rgba(210, 248, 132, 0.3);
        }

        .badge-status.running {
            background-color: var(--accent-cyan-bg);
            color: var(--accent-cyan);
            border: 1px solid rgba(86, 212, 245, 0.3);
            animation: pulse 1.5s infinite;
        }

        .badge-status.queued {
            background-color: var(--bg-canvas);
            color: var(--text-muted);
            border: 1px solid var(--border);
        }

        .suite-name {
            font-size: 14px;
            font-weight: 600;
        }

        .suite-metrics-pill {
            font-family: var(--font-mono);
            font-size: 12px;
            color: var(--text-secondary);
            display: flex;
            gap: 14px;
            align-items: center;
        }

        .metric-highlight {
            color: var(--accent-lime);
            font-weight: 600;
        }

        .suite-collapse-body {
            padding: 16px 20px;
            border-top: 1px solid var(--border);
            background-color: #0c1220;
        }

        .diff-table {
            width: 100%;
            border-collapse: collapse;
            font-size: 13px;
            margin-top: 8px;
        }

        .diff-table th, .diff-table td {
            padding: 10px 14px;
            border-bottom: 1px solid var(--border);
            text-align: left;
        }

        .diff-table th {
            font-family: var(--font-mono);
            font-size: 11px;
            color: var(--text-muted);
            text-transform: uppercase;
        }

        .diff-table tr:hover td {
            background-color: var(--bg-card-hover);
        }

        .delta-badge {
            font-family: var(--font-mono);
            font-size: 11px;
            font-weight: 700;
            padding: 2px 8px;
            border-radius: 4px;
            display: inline-block;
        }

        .delta-badge.improved {
            background-color: var(--accent-lime-bg);
            color: var(--accent-lime);
        }

        .delta-badge.regressed {
            background-color: var(--accent-red-bg);
            color: var(--accent-red);
        }

        .delta-badge.stable {
            background-color: var(--bg-canvas);
            color: var(--text-secondary);
        }

        /* Flamegraph Container */
        .flamegraph-viewer {
            width: 100%;
            min-height: 620px;
            border: 1px solid var(--border);
            border-radius: 8px;
            background-color: #07090f;
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }

        .flamegraph-toolbar {
            padding: 10px 16px;
            background-color: var(--bg-card);
            border-bottom: 1px solid var(--border);
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 16px;
            flex-wrap: wrap;
        }

        .flamegraph-frame {
            flex: 1;
            width: 100%;
            min-height: 580px;
            border: none;
            background: #ffffff;
        }

        /* Sidebar Log Feed */
        .sidebar {
            display: flex;
            flex-direction: column;
            gap: 20px;
        }

        .sidebar-card {
            background-color: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 16px;
            display: flex;
            flex-direction: column;
            height: 100%;
        }

        .log-terminal {
            background-color: #07090f;
            border: 1px solid var(--border);
            border-radius: 6px;
            padding: 12px;
            font-family: var(--font-mono);
            font-size: 11px;
            color: #a0aec0;
            height: 520px;
            overflow-y: auto;
            white-space: pre-wrap;
            word-break: break-all;
        }

        /* Footer */
        .footer {
            border-top: 1px solid var(--border);
            padding: 10px 32px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            font-size: 12px;
            color: var(--text-muted);
            font-family: var(--font-mono);
            background-color: var(--bg-canvas);
        }

        @keyframes pulse {
            0% { opacity: 0.8; }
            50% { opacity: 1; }
            100% { opacity: 0.8; }
        }
    </style>
</head>
<body>
    <div id="app" x-data="benchApp()" x-init="init()">
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
            <div class="nav-right">
                <div class="status-dot" :class="wsConnected ? '' : 'disconnected'"></div>
                <span style="font-size: 12px; font-family: var(--font-mono); color: var(--text-secondary);" x-text="wsConnected ? 'CONNECTED :7787' : 'CONNECTING...'"></span>
                <button class="btn-ghost" @click="refreshStatus()">REFRESH</button>
            </div>
        </nav>

        <!-- Main Container -->
        <div class="container">
            <main>
                <!-- 1. BENCHMARK RUNNER VIEW -->
                <div x-show="activeTab === 'runner'">
                    <div class="page-title">
                        <span style="width: 14px; height: 14px; background: var(--accent-lavender); border-radius: 3px; display: inline-block;"></span>
                        <span>Relay Benchmarking & Analysis</span>
                    </div>

                    <!-- 6 KPI Horizontal Cards (One Row) -->
                    <div class="kpi-grid">
                        <div class="kpi-card">
                            <div class="kpi-header">✓ COMPLETED</div>
                            <div class="kpi-value" x-text="status.completed_suites_count + ' / ' + status.total_suites"></div>
                            <div class="kpi-footer">SUITES DONE</div>
                        </div>
                        <div class="kpi-card">
                            <div class="kpi-header">⚡ ACTIVE TEST</div>
                            <div class="kpi-value" style="font-size: 15px; color: var(--accent-cyan);" x-text="status.current_suite || (status.is_running ? 'RUNNING' : 'IDLE')"></div>
                            <div class="kpi-footer">CURRENT STEP</div>
                        </div>
                        <div class="kpi-card">
                            <div class="kpi-header">⏱ ELAPSED</div>
                            <div class="kpi-value" x-text="status.elapsed_secs.toFixed(1) + 's'"></div>
                            <div class="kpi-footer">TOTAL TIME</div>
                        </div>
                        <div class="kpi-card">
                            <div class="kpi-header">🚀 PEAK TPS</div>
                            <div class="kpi-value" style="color: var(--accent-lime);" x-text="status.peak_tps > 0 ? Math.round(status.peak_tps) : '-'"></div>
                            <div class="kpi-footer">THROUGHPUT</div>
                        </div>
                        <div class="kpi-card">
                            <div class="kpi-header">🎯 P99 LATENCY</div>
                            <div class="kpi-value" x-text="status.best_p99_ms ? status.best_p99_ms.toFixed(2) + 'ms' : '-'"></div>
                            <div class="kpi-footer">BEST P99</div>
                        </div>
                        <div class="kpi-card">
                            <div class="kpi-header">💾 MEMORY RSS</div>
                            <div class="kpi-value" style="color: var(--accent-lavender);" x-text="status.peak_rss_mb > 0 ? status.peak_rss_mb.toFixed(1) + ' MB' : '-'"></div>
                            <div class="kpi-footer">RELAY RESIDENT</div>
                        </div>
                    </div>

                    <!-- Progress Bar Section -->
                    <div class="progress-section">
                        <div class="section-comment" x-text="'// live progress · ' + Math.round((status.completed_suites_count / status.total_suites) * 100) + '% completed · ' + status.current_step"></div>
                        <div class="progress-bar-bg">
                            <div class="progress-bar-fill" :style="'width: ' + Math.round((status.completed_suites_count / status.total_suites) * 100) + '%'"></div>
                        </div>
                    </div>

                    <!-- Controls & Configuration Card -->
                    <div class="config-card">
                        <div class="config-header">
                            <div class="section-comment">// test configuration & execution</div>
                            <div style="display: flex; gap: 12px; flex-wrap: wrap;">
                                <!-- Target Mode Toggle -->
                                <div class="mode-toggle">
                                    <button class="mode-btn" :class="currentTargetMode === 'source' ? 'active' : ''" @click="setTargetMode('source')">🏗 Build from Source</button>
                                    <button class="mode-btn" :class="currentTargetMode === 'live' ? 'active' : ''" @click="setTargetMode('live')">⚡ Live Relay</button>
                                </div>

                                <!-- Test Mode Toggle -->
                                <div class="mode-toggle" x-show="currentTargetMode === 'source'">
                                    <button class="mode-btn" :class="currentTestMode === 'single' ? 'active' : ''" @click="setTestMode('single')">Single Test</button>
                                    <button class="mode-btn" :class="currentTestMode === 'compare' ? 'active' : ''" @click="setTestMode('compare')">Comparison A/B</button>
                                </div>
                            </div>
                        </div>

                        <!-- Source Build Controls -->
                        <div x-show="currentTargetMode === 'source'" class="controls-row">
                            <!-- Initial (Base) -->
                            <div class="control-group" x-show="currentTestMode === 'compare'">
                                <label>Initial (Base Branch & Commit)</label>
                                <div style="display: flex; gap: 8px;">
                                    <select x-model="selectedBaseBranch" @change="onBaseBranchChanged()" style="width: 140px;">
                                        <template x-for="b in branches" :key="b">
                                            <option :value="b" x-text="b"></option>
                                        </template>
                                    </select>
                                    <select x-model="selectedBaseCommit">
                                        <template x-for="c in baseCommits" :key="c.hash">
                                            <option :value="c.hash" x-text="c.short_hash + ' - ' + c.message.substring(0, 32)"></option>
                                        </template>
                                    </select>
                                </div>
                            </div>

                            <!-- Final (Target) -->
                            <div class="control-group">
                                <label x-text="currentTestMode === 'compare' ? (compareCurrent ? 'Final (Target): Current Codebase' : 'Final (Target Branch & Commit)') : 'Branch & Commit to Benchmark'"></label>
                                <div style="display: flex; gap: 8px;">
                                    <select x-model="selectedTargetBranch" @change="onTargetBranchChanged()" :disabled="currentTestMode === 'compare' && compareCurrent" style="width: 140px;">
                                        <template x-for="b in branches" :key="b">
                                            <option :value="b" x-text="b"></option>
                                        </template>
                                    </select>
                                    <select x-model="selectedTargetCommit" :disabled="currentTestMode === 'compare' && compareCurrent">
                                        <template x-for="c in targetCommits" :key="c.hash">
                                            <option :value="c.hash" x-text="c.short_hash + ' - ' + c.message.substring(0, 32)"></option>
                                        </template>
                                    </select>
                                </div>
                            </div>

                            <div>
                                <button class="btn-lime" :disabled="status.is_running" @click="triggerRun()">
                                    <span x-show="status.is_running">⏳ RUNNING...</span>
                                    <span x-show="!status.is_running">⚡ RUN BENCHMARK</span>
                                </button>
                            </div>
                        </div>

                        <!-- Live Testing Controls -->
                        <div x-show="currentTargetMode === 'live'" class="controls-row">
                            <div class="control-group" style="grid-column: span 2;">
                                <label>Target Relay URL (ws://...)</label>
                                <input type="text" x-model="liveUrl" placeholder="ws://localhost:7777">
                            </div>
                            <div>
                                <button class="btn-lime" :disabled="status.is_running" @click="triggerRun()">
                                    <span x-show="status.is_running">⏳ TESTING...</span>
                                    <span x-show="!status.is_running">⚡ START LIVE TEST</span>
                                </button>
                            </div>
                        </div>

                        <!-- Checkboxes -->
                        <div style="display: flex; gap: 24px; margin-top: 14px; flex-wrap: wrap;">
                            <label class="checkbox-label" x-show="currentTargetMode === 'source' && currentTestMode === 'compare'">
                                <input type="checkbox" x-model="compareCurrent"> Compare Current Codebase (Stash changes)
                            </label>
                            <label class="checkbox-label" x-show="currentTargetMode === 'source'">
                                <input type="checkbox" x-model="highPerformance"> High-Performance Build (make -j$(nproc))
                            </label>
                            <label class="checkbox-label">
                                <input type="checkbox" x-model="skipHeavy"> Skip 1M Event Heavy Storage Test
                            </label>
                            <label class="checkbox-label" x-show="currentTargetMode === 'source'">
                                <input type="checkbox" x-model="flamegraph"> Generate CPU Flamegraph
                            </label>
                        </div>
                    </div>

                    <!-- Progressive Collapsible Test Suites -->
                    <div class="section-comment">// test suites & immediate results (click to expand / auto-expands on completion)</div>
                    <div class="suite-list">
                        <template x-for="(s, idx) in suitesConfig" :key="s.id">
                            <div class="suite-card" :class="status.current_suite === s.id ? 'running' : ''">
                                <div class="suite-card-header" @click="toggleSuite(s.id)">
                                    <div class="suite-info">
                                        <span x-show="completedMap[s.id]" class="badge-status completed">✓ COMPLETED</span>
                                        <span x-show="status.current_suite === s.id && !completedMap[s.id]" class="badge-status running">⚡ TESTING</span>
                                        <span x-show="!completedMap[s.id] && status.current_suite !== s.id" class="badge-status queued">○ QUEUED</span>
                                        <span class="suite-name" x-text="s.name"></span>
                                    </div>
                                    <div class="suite-metrics-pill">
                                        <template x-if="completedMap[s.id]">
                                            <div style="display: flex; gap: 12px; align-items: center;">
                                                <span class="metric-highlight" x-text="completedMap[s.id].throughput ? completedMap[s.id].throughput.toFixed(0) + ' ' + (completedMap[s.id].throughput_label || 'ops/s') : ''"></span>
                                                <span x-show="completedMap[s.id].p50_ms" x-text="'P50: ' + (completedMap[s.id].p50_ms ? completedMap[s.id].p50_ms.toFixed(2) + 'ms' : '')"></span>
                                                <span x-show="completedMap[s.id].p99_ms" x-text="'P99: ' + (completedMap[s.id].p99_ms ? completedMap[s.id].p99_ms.toFixed(2) + 'ms' : '')"></span>
                                                <span x-show="completedMap[s.id].memory_rss_mb" style="color: var(--accent-lavender);" x-text="'RSS: ' + (completedMap[s.id].memory_rss_mb ? completedMap[s.id].memory_rss_mb.toFixed(1) + 'MB' : '')"></span>
                                                <span x-text="completedMap[s.id].elapsed_secs.toFixed(2) + 's'"></span>
                                            </div>
                                        </template>
                                        <template x-if="!completedMap[s.id]">
                                            <span style="color: var(--text-muted);" x-text="status.current_suite === s.id ? 'Testing...' : 'Awaiting runner...'"></span>
                                        </template>
                                        <span style="color: var(--text-muted); font-size: 11px;" x-text="expandedSuites[s.id] ? '▲' : '▼'"></span>
                                    </div>
                                </div>

                                <!-- Collapsible Body -->
                                <div x-show="expandedSuites[s.id]" class="suite-collapse-body">
                                    <template x-if="completedMap[s.id]">
                                        <div>
                                            <!-- Metric Pills -->
                                            <div style="display: flex; gap: 8px; flex-wrap: wrap; margin-bottom: 12px;" v-if="completedMap[s.id].metrics">
                                                <template x-for="(v, k) in completedMap[s.id].metrics" :key="k">
                                                    <div x-show="typeof v === 'number'" style="background: var(--bg-canvas); border: 1px solid var(--border); padding: 3px 8px; border-radius: 4px; font-family: var(--font-mono); font-size: 11px;">
                                                        <span style="color: var(--text-muted);" x-text="k.replace(/_/g, ' ') + ':'"></span>
                                                        <span style="color: var(--accent-lime); font-weight: 600;" x-text="typeof v === 'number' ? v.toFixed(2) : v"></span>
                                                    </div>
                                                </template>
                                            </div>
                                            <!-- Raw Output -->
                                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 6px;">
                                                <span class="section-comment" style="margin: 0;">// terminal output</span>
                                                <button class="btn-ghost" style="padding: 2px 8px; font-size: 11px;" @click="copyToClipboard(completedMap[s.id].log_output)">📋 Copy</button>
                                            </div>
                                            <pre style="background: #07090f; border: 1px solid var(--border); border-radius: 6px; padding: 12px; font-family: var(--font-mono); font-size: 11px; color: #a0aec0; white-space: pre-wrap; line-height: 1.4; margin: 0;" x-text="completedMap[s.id].log_output"></pre>
                                        </div>
                                    </template>
                                    <template x-if="!completedMap[s.id]">
                                        <div style="color: var(--text-muted); font-size: 12px; font-family: var(--font-mono);">
                                            Suite is currently running or queued. Results will appear automatically upon completion.
                                        </div>
                                    </template>
                                </div>
                            </div>
                        </template>
                    </div>
                </div>

                <!-- 2. COMPARISON VIEW -->
                <div x-show="activeTab === 'comparison'">
                    <div class="page-title">
                        <span style="width: 14px; height: 14px; background: var(--accent-lime); border-radius: 3px; display: inline-block;"></span>
                        <span>A/B Benchmark Comparison</span>
                    </div>

                    <div class="config-card">
                        <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 16px;">
                            <div>
                                <div class="section-comment">// select historical comparison run</div>
                                <select x-model="selectedComparisonReportId" @change="loadComparisonReport(selectedComparisonReportId)" style="width: 380px;">
                                    <template x-for="r in pastComparisonReports" :key="r">
                                        <option :value="r" x-text="r"></option>
                                    </template>
                                </select>
                            </div>
                            <div v-if="comparisonReportData" style="font-family: var(--font-mono); font-size: 12px; color: var(--text-secondary);">
                                Initial: <span style="color: var(--accent-lavender);" x-text="comparisonReportData ? comparisonReportData.base_ref : ''"></span> &nbsp;|&nbsp;
                                Final: <span style="color: var(--accent-lime);" x-text="comparisonReportData ? comparisonReportData.target_ref : ''"></span>
                            </div>
                        </div>
                    </div>

                    <div class="config-card" x-show="comparisonReportData && comparisonReportData.deltas">
                        <div class="section-comment">// performance deltas & deterministic metrics</div>
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

                <!-- 3. PAST RUNS VIEW (Single and Comparison with Paired C1 vs C2 Layout) -->
                <div x-show="activeTab === 'past'">
                    <div class="page-title">
                        <span style="width: 14px; height: 14px; background: var(--accent-lavender); border-radius: 3px; display: inline-block;"></span>
                        <span>Historical Test Reports</span>
                    </div>

                    <div class="config-card">
                        <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 16px;">
                            <div>
                                <div class="section-comment">// select past benchmark run</div>
                                <select x-model="selectedPastReportId" @change="loadPastReport(selectedPastReportId)" style="width: 420px;">
                                    <template x-for="r in pastReports" :key="r">
                                        <option :value="r" x-text="r.includes('compare') ? '[A/B COMPARISON] ' + r : '[SINGLE RUN] ' + r"></option>
                                    </template>
                                </select>
                            </div>
                            <div v-if="pastReportData" style="font-family: var(--font-mono); font-size: 12px; color: var(--text-secondary);">
                                Target: <span style="color: var(--accent-lime);" x-text="pastReportData ? pastReportData.target_ref : ''"></span> &nbsp;|&nbsp;
                                Date: <span style="color: var(--accent-lavender);" x-text="pastReportData && pastReportData.timestamp ? new Date(pastReportData.timestamp).toLocaleString() : ''"></span>
                            </div>
                        </div>

                        <!-- Subview Navigation -->
                        <div style="display: flex; gap: 8px; margin-top: 18px; flex-wrap: wrap;" x-show="pastReportData">
                            <button class="btn-ghost" :class="pastSubView === 'paired' ? 'active' : ''" @click="pastSubView = 'paired'">
                                <span x-text="pastReportData && pastReportData.deltas ? '📑 Paired (C1 vs C2 per Suite)' : '📑 13-Suite Breakdown'"></span>
                            </button>
                            <button class="btn-ghost" :class="pastSubView === 'deltas' ? 'active' : ''" x-show="pastReportData && pastReportData.deltas" @click="pastSubView = 'deltas'">
                                <span x-text="'📊 Comparison Deltas (' + (pastReportData && pastReportData.deltas ? pastReportData.deltas.length : 0) + ')'"></span>
                            </button>
                            <button class="btn-ghost" :class="pastSubView === 'markdown' ? 'active' : ''" @click="loadPastMarkdown()">
                                📝 Markdown Report
                            </button>
                            <button class="btn-ghost" :class="pastSubView === 'json' ? 'active' : ''" @click="pastSubView = 'json'">
                                💾 report.json
                            </button>
                        </div>
                    </div>

                    <!-- Subview 1: Paired C1 vs C2 Layout -->
                    <div x-show="pastSubView === 'paired'" x-if="pastReportData">
                        <!-- Comparison Run: Paired side-by-side cards -->
                        <div x-show="pastReportData && pastReportData.deltas" style="display: flex; flex-direction: column; gap: 20px;">
                            <template x-for="(s, idx) in (pastReportData && pastReportData.target_report ? pastReportData.target_report.suites : [])" :key="s.id">
                                <div class="suite-card" style="padding: 20px;">
                                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; border-bottom: 1px solid var(--border); padding-bottom: 12px; flex-wrap: wrap; gap: 8px;">
                                        <div style="font-size: 16px; font-weight: 700; color: var(--text-primary);">
                                            <span style="color: var(--accent-lavender); margin-right: 6px;" x-text="'#' + (idx + 1)"></span>
                                            <span x-text="s.name"></span>
                                        </div>
                                        <span class="badge-status completed">COMPLETED</span>
                                    </div>

                                    <!-- Paired Grid: Initial (C1) vs Final (C2) -->
                                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 20px;">
                                        <!-- Initial C1 -->
                                        <div style="background: #090e1a; border: 1px solid rgba(138, 153, 252, 0.4); border-radius: 6px; padding: 16px;">
                                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                                                <span class="badge-status" style="background: #181d33; color: var(--accent-lavender); border: 1px solid rgba(138, 153, 252, 0.5);" x-text="'Initial (C1): ' + pastReportData.base_ref"></span>
                                                <span style="font-family: var(--font-mono); font-size: 12px; color: var(--text-muted);" x-text="getMatchingBaseSuite(s.id) ? getMatchingBaseSuite(s.id).elapsed_secs.toFixed(2) + 's' : '-'"></span>
                                            </div>
                                            <template x-if="getMatchingBaseSuite(s.id)">
                                                <div>
                                                    <div style="font-family: var(--font-mono); font-size: 13px; margin-bottom: 10px;">
                                                        <span style="color: var(--accent-lavender); font-weight: 700;" x-text="getMatchingBaseSuite(s.id).throughput ? getMatchingBaseSuite(s.id).throughput.toFixed(1) + ' ' + (getMatchingBaseSuite(s.id).throughput_label || 'ops/s') : '-'"></span>
                                                        <span style="margin-left: 8px; color: var(--text-secondary);" x-show="getMatchingBaseSuite(s.id).p50_ms" x-text="'P50: ' + (getMatchingBaseSuite(s.id).p50_ms ? getMatchingBaseSuite(s.id).p50_ms.toFixed(2) + 'ms' : '')"></span>
                                                    </div>
                                                    <div style="display: flex; gap: 6px; flex-wrap: wrap; margin-bottom: 10px;">
                                                        <template x-for="(v, k) in getMatchingBaseSuite(s.id).metrics" :key="k">
                                                            <div x-show="typeof v === 'number'" style="background: var(--bg-canvas); border: 1px solid var(--border); padding: 2px 6px; border-radius: 4px; font-family: var(--font-mono); font-size: 11px;">
                                                                <span style="color: var(--text-muted);" x-text="k.replace(/_/g, ' ') + ':'"></span>
                                                                <span style="color: var(--accent-lavender); font-weight: 600;" x-text="typeof v === 'number' ? v.toFixed(2) : v"></span>
                                                            </div>
                                                        </template>
                                                    </div>
                                                    <pre style="background: #07090f; border: 1px solid var(--border); border-radius: 6px; padding: 10px; font-family: var(--font-mono); font-size: 11px; color: #a0aec0; white-space: pre-wrap; line-height: 1.4; margin: 0;" x-text="getMatchingBaseSuite(s.id).log_output"></pre>
                                                </div>
                                            </template>
                                        </div>

                                        <!-- Final C2 -->
                                        <div style="background: #091217; border: 1px solid rgba(210, 248, 132, 0.4); border-radius: 6px; padding: 16px;">
                                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                                                <span class="badge-status" style="background: #1c2712; color: var(--accent-lime); border: 1px solid rgba(210, 248, 132, 0.5);" x-text="'Final (C2): ' + pastReportData.target_ref"></span>
                                                <span style="font-family: var(--font-mono); font-size: 12px; color: var(--text-muted);" x-text="s.elapsed_secs.toFixed(2) + 's'"></span>
                                            </div>
                                            <div>
                                                <div style="font-family: var(--font-mono); font-size: 13px; margin-bottom: 10px;">
                                                    <span style="color: var(--accent-lime); font-weight: 700;" x-text="s.throughput ? s.throughput.toFixed(1) + ' ' + (s.throughput_label || 'ops/s') : '-'"></span>
                                                    <span style="margin-left: 8px; color: var(--text-secondary);" x-show="s.p50_ms" x-text="'P50: ' + (s.p50_ms ? s.p50_ms.toFixed(2) + 'ms' : '')"></span>
                                                </div>
                                                <div style="display: flex; gap: 6px; flex-wrap: wrap; margin-bottom: 10px;">
                                                    <template x-for="(v, k) in s.metrics" :key="k">
                                                        <div x-show="typeof v === 'number'" style="background: var(--bg-canvas); border: 1px solid var(--border); padding: 2px 6px; border-radius: 4px; font-family: var(--font-mono); font-size: 11px;">
                                                            <span style="color: var(--text-muted);" x-text="k.replace(/_/g, ' ') + ':'"></span>
                                                            <span style="color: var(--accent-lime); font-weight: 600;" x-text="typeof v === 'number' ? v.toFixed(2) : v"></span>
                                                        </div>
                                                    </template>
                                                </div>
                                                <pre style="background: #07090f; border: 1px solid var(--border); border-radius: 6px; padding: 10px; font-family: var(--font-mono); font-size: 11px; color: #a0aec0; white-space: pre-wrap; line-height: 1.4; margin: 0;" x-text="s.log_output"></pre>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </template>
                        </div>

                        <!-- Single Run: 13 Suite Cards -->
                        <div x-show="pastReportData && !pastReportData.deltas" style="display: flex; flex-direction: column; gap: 16px;">
                            <template x-for="(s, idx) in (pastReportData ? pastReportData.suites : [])" :key="s.id">
                                <div class="suite-card" style="padding: 18px;">
                                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 14px; flex-wrap: wrap; gap: 8px;">
                                        <div style="display: flex; align-items: center; gap: 12px;">
                                            <span class="badge-status completed" x-text="'✓ #' + (idx + 1) + ' ' + s.name"></span>
                                            <span style="font-size: 12px; color: var(--text-muted); font-family: var(--font-mono);" x-text="s.elapsed_secs.toFixed(2) + 's'"></span>
                                        </div>
                                        <div style="font-family: var(--font-mono); font-size: 12px;">
                                            <span class="metric-highlight" x-text="s.throughput ? s.throughput.toFixed(1) + ' ' + (s.throughput_label || 'ops/s') : '-'"></span>
                                            <span v-show="s.p50_ms" style="color: var(--text-secondary); margin-left: 12px;" x-text="s.p50_ms ? 'P50: ' + s.p50_ms.toFixed(2) + 'ms' : ''"></span>
                                            <span v-show="s.memory_rss_mb" style="color: var(--accent-lavender); margin-left: 10px;" x-text="s.memory_rss_mb ? 'RSS: ' + s.memory_rss_mb.toFixed(1) + 'MB' : ''"></span>
                                        </div>
                                    </div>
                                    <pre style="background: #07090f; border: 1px solid var(--border); border-radius: 6px; padding: 12px; font-family: var(--font-mono); font-size: 11px; color: #a0aec0; white-space: pre-wrap; line-height: 1.4; margin: 0;" x-text="s.log_output"></pre>
                                </div>
                            </template>
                        </div>
                    </div>

                    <!-- Subview 2: Deltas Table -->
                    <div x-show="pastSubView === 'deltas'" class="config-card">
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
                                <template x-for="d in (pastReportData && pastReportData.deltas ? pastReportData.deltas : [])" :key="d.metric">
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

                    <!-- Subview 3: Markdown Report View -->
                    <div x-show="pastSubView === 'markdown'" class="config-card">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                            <span class="section-comment">// GitHub-Flavored Markdown Report</span>
                            <button class="btn-ghost" @click="copyToClipboard(pastMarkdownText)">📋 Copy Markdown</button>
                        </div>
                        <pre style="background: #07090f; padding: 16px; border-radius: 6px; border: 1px solid var(--border); font-family: var(--font-mono); font-size: 12px; color: #a0aec0; white-space: pre-wrap; line-height: 1.5;" x-text="pastMarkdownText"></pre>
                    </div>

                    <!-- Subview 4: Raw JSON View -->
                    <div x-show="pastSubView === 'json'" class="config-card">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                            <span class="section-comment">// Full Machine-Readable JSON</span>
                            <button class="btn-ghost" @click="copyToClipboard(JSON.stringify(pastReportData, null, 2))">📋 Copy JSON</button>
                        </div>
                        <pre style="background: #07090f; padding: 16px; border-radius: 6px; border: 1px solid var(--border); font-family: var(--font-mono); font-size: 12px; color: #a0aec0; white-space: pre-wrap; max-height: 600px; overflow-y: auto;" x-text="JSON.stringify(pastReportData, null, 2)"></pre>
                    </div>
                </div>

                <!-- 4. ANALYTICS VIEW -->
                <div x-show="activeTab === 'analytics'">
                    <div class="page-title">
                        <span style="width: 14px; height: 14px; background: var(--accent-cyan); border-radius: 3px; display: inline-block;"></span>
                        <span>Throughput & Latency Distribution</span>
                    </div>

                    <div class="config-card">
                        <div class="section-comment">// suite throughput comparison (events/sec & req/sec)</div>
                        <div style="display: flex; flex-direction: column; gap: 14px; margin-top: 16px;">
                            <template x-for="s in analyticsSuites" :key="s.name">
                                <div>
                                    <div style="display: flex; justify-content: space-between; font-size: 13px; margin-bottom: 4px;">
                                        <span x-text="s.name"></span>
                                        <span style="font-family: var(--font-mono); color: var(--accent-lime);" x-text="s.throughput.toFixed(0) + ' ' + (s.throughput_label || 'ops/s')"></span>
                                    </div>
                                    <div style="width: 100%; height: 10px; background-color: var(--bg-canvas); border-radius: 5px; overflow: hidden; border: 1px solid var(--border);">
                                        <div :style="'width: ' + Math.max(5, (s.throughput / maxAnalyticsTps) * 100) + '%; height: 100%; background-color: var(--accent-lime);'"></div>
                                    </div>
                                </div>
                            </template>
                        </div>
                    </div>
                </div>

                <!-- 5. FLAMEGRAPH VIEW -->
                <div x-show="activeTab === 'flamegraph'">
                    <div class="page-title">
                        <span style="width: 14px; height: 14px; background: #ff9800; border-radius: 3px; display: inline-block;"></span>
                        <span>CPU Flamegraph Inspection</span>
                    </div>

                    <div class="flamegraph-viewer">
                        <div class="flamegraph-toolbar">
                            <div style="display: flex; align-items: center; gap: 12px; flex-wrap: wrap;">
                                <span class="section-comment" style="margin: 0;">// benchmark run</span>
                                <select x-model="selectedFlamegraphReportId" @change="onFlamegraphReportChanged()" style="width: 320px; padding: 6px 10px; font-size: 12px;">
                                    <template x-for="r in pastReports" :key="r">
                                        <option :value="r" x-text="r"></option>
                                    </template>
                                </select>

                                <span class="section-comment" style="margin: 0;">// available svg</span>
                                <select x-model="selectedFlamegraphFile" @change="onFlamegraphFileChanged()" style="width: 240px; padding: 6px 10px; font-size: 12px;">
                                    <template x-for="f in flamegraphFiles" :key="f.file">
                                        <option :value="f.file" x-text="f.label"></option>
                                    </template>
                                </select>
                            </div>
                            <div style="display: flex; gap: 8px;">
                                <button class="btn-ghost" @click="refreshFlamegraph()">⟳ REFRESH</button>
                                <a x-show="selectedFlamegraphFile" class="btn-ghost" style="text-decoration: none;" :href="flamegraphUrl" target="_blank" download>⤓ DOWNLOAD SVG</a>
                            </div>
                        </div>
                        <div x-show="flamegraphFiles.length === 0" style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 500px; text-align: center; padding: 40px;">
                            <div style="font-size: 36px; margin-bottom: 12px;">🔥</div>
                            <div style="font-size: 18px; font-weight: 700; color: var(--text-primary); margin-bottom: 8px;">No Flamegraph Found in This Run</div>
                            <div style="font-size: 13px; color: var(--text-secondary); max-width: 520px; margin-bottom: 24px; line-height: 1.6;">
                                CPU profiling was not enabled for this benchmark run. Make sure <strong>"Generate CPU Flamegraph"</strong> is checked in the Benchmark tab, or pass <code>--flamegraph</code> on the CLI.
                            </div>
                            <button class="btn-lime" @click="activeTab = 'runner'">⚡ Go to Benchmark Runner</button>
                        </div>
                        <iframe x-show="flamegraphFiles.length > 0" class="flamegraph-frame" :src="flamegraphUrl"></iframe>
                    </div>
                </div>
            </main>

            <!-- Right Sidebar (Telemetry & Live Log Stream) -->
            <aside class="sidebar">
                <div class="sidebar-card">
                    <div class="section-comment">// context & live telemetry</div>
                    <div style="font-size: 13px; font-weight: 600; margin-bottom: 12px; color: var(--accent-cyan);" x-text="status.current_step"></div>
                    <div class="log-terminal" id="log-terminal">
                        <template x-for="(line, i) in logs" :key="i">
                            <div x-text="line"></div>
                        </template>
                    </div>
                </div>
            </aside>
        </div>

        <!-- Bottom Status Bar -->
        <footer class="footer">
            <div>
                <span class="brand-badge" style="font-size: 10px; padding: 2px 6px;">STRFRY</span>
                <span x-text="status.is_running ? 'Benchmark Active · Running' : 'Ready'"></span>
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

                selectedFlamegraphReportId: '',
                flamegraphFiles: [],
                selectedFlamegraphFile: '',
                flamegraphUrl: 'about:blank',

                logs: ['Connecting to live benchmark stream...'],
                wsConnected: false,

                get analyticsSuites() {
                    const suites = Object.values(this.completedMap).filter(s => s && s.throughput);
                    if (suites.length > 0) return suites;
                    if (this.pastReportData) {
                        return (this.pastReportData.suites || (this.pastReportData.target_report && this.pastReportData.target_report.suites) || []).filter(s => s && s.throughput);
                    }
                    return [];
                },

                get maxAnalyticsTps() {
                    const tpsArr = this.analyticsSuites.map(s => s.throughput);
                    return tpsArr.length > 0 ? Math.max(...tpsArr) : 1000;
                },

                setTargetMode(mode) {
                    this.currentTargetMode = mode;
                },

                setTestMode(mode) {
                    this.currentTestMode = mode;
                },

                toggleSuite(id) {
                    this.expandedSuites[id] = !this.expandedSuites[id];
                },

                getMatchingBaseSuite(id) {
                    if (!this.pastReportData || !this.pastReportData.base_report) return null;
                    return this.pastReportData.base_report.suites.find(s => s.id === id);
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
                    } catch (e) {
                        console.error("Failed to fetch branches", e);
                    }
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
                    // Instantly update UI to progressing state
                    this.status.is_running = true;
                    this.status.current_step = 'Initiating build and test execution...';
                    this.status.current_suite = 'Building strfry...';
                    this.status.completed_suites_count = 0;
                    this.status.elapsed_secs = 0;
                    this.completedMap = {};
                    this.expandedSuites = {};
                    this.logs = ['[BENCH] Triggering benchmark run from Web UI...'];

                    const targetCommitVal = (this.currentTestMode === 'compare' && this.compareCurrent && this.currentTargetMode === 'source')
                        ? "current-codebase"
                        : this.selectedTargetCommit;

                    const req = {
                        target_mode: this.currentTargetMode,
                        url: this.liveUrl,
                        test_type: this.currentTestMode,
                        base: this.selectedBaseCommit,
                        target: targetCommitVal,
                        current: this.currentTestMode === 'compare' && this.compareCurrent,
                        high_performance: this.highPerformance,
                        skip_heavy: this.skipHeavy,
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
                                // Auto-expand on completion
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
                    } catch (e) {
                        console.error("Failed to load comparison report", e);
                    }
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
                    if (this.pastReports.length > 0 && !this.pastReportData) {
                        await this.loadPastReport(this.pastReports[0]);
                    }
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
                        this.pastSubView = this.pastReportData.deltas ? 'paired' : 'paired';
                    } catch (e) {
                        console.error("Failed to load past report", e);
                    }
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
                            else if (f === 'final_flamegraph.svg' || f === 'target_flamegraph.svg') label = 'Final Flamegraph (Target)';
                            else if (f === 'initial_flamegraph.svg' || f === 'base_flamegraph.svg') label = 'Initial Flamegraph (Base)';
                            this.flamegraphFiles.push({ file: f, label });
                        });

                        if (this.flamegraphFiles.length > 0) {
                            this.selectedFlamegraphFile = this.flamegraphFiles[0].file;
                        } else {
                            this.selectedFlamegraphFile = '';
                        }
                        this.onFlamegraphFileChanged();
                    } catch (e) {
                        console.error("Failed to load flamegraphs", e);
                    }
                },

                onFlamegraphFileChanged() {
                    if (!this.selectedFlamegraphReportId || !this.selectedFlamegraphFile) {
                        this.flamegraphUrl = 'about:blank';
                    } else {
                        this.flamegraphUrl = `/api/reports/${encodeURIComponent(this.selectedFlamegraphReportId)}/${encodeURIComponent(this.selectedFlamegraphFile)}`;
                    }
                },

                async refreshFlamegraph() {
                    await this.loadReportsList();
                    await this.onFlamegraphReportChanged();
                },

                copyToClipboard(text) {
                    navigator.clipboard.writeText(text);
                },

                connectWs() {
                    const protocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
                    const ws = new WebSocket(`${protocol}//${location.host}/api/ws`);

                    ws.onopen = () => {
                        this.wsConnected = true;
                    };

                    ws.onmessage = (event) => {
                        const data = JSON.parse(event.data);
                        if (data.type === 'telemetry') {
                            this.updateTelemetry(data.status, data.completed_suites);
                        } else if (data.type === 'log') {
                            this.logs.push(data.line);
                            setTimeout(() => {
                                const term = document.getElementById('log-terminal');
                                if (term) term.scrollTop = term.scrollHeight;
                            }, 50);
                        }
                    };

                    ws.onclose = () => {
                        this.wsConnected = false;
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
