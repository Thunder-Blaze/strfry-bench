pub const VUE_JS: &str = include_str!("vue.min.js");

pub const RENDERED_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Strfry Bench - Performance & Comparison Dashboard</title>
    <script src="/assets/vue.min.js"></script>
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

        /* Section Comments */
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

        /* KPI Balanced 6-Card Grid */
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

        /* 3-Column Stats Grid */
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 12px;
            margin-bottom: 14px;
        }

        @media (max-width: 900px) {
            .stats-grid {
                grid-template-columns: 1fr;
            }
        }

        .stat-box {
            background-color: var(--bg-canvas);
            border: 1px solid var(--border);
            border-radius: 6px;
            padding: 10px 14px;
        }

        .stat-box-title {
            font-size: 10px;
            font-family: var(--font-mono);
            text-transform: uppercase;
            color: var(--text-muted);
            margin-bottom: 4px;
        }

        .stat-box-value {
            font-size: 14px;
            font-family: var(--font-mono);
            font-weight: 700;
            color: var(--text-primary);
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
    <div id="app">
        <!-- Top Navigation -->
        <nav class="navbar">
            <div class="nav-left">
                <div class="brand-badge">STRFRY</div>
                <div class="nav-links">
                    <a class="nav-link" :class="{ active: activeTab === 'runner' }" @click="activeTab = 'runner'">BENCHMARK</a>
                    <a class="nav-link" :class="{ active: activeTab === 'comparison' }" @click="openComparisonTab()">COMPARISON</a>
                    <a class="nav-link" :class="{ active: activeTab === 'past' }" @click="openPastTab()">PAST RUNS</a>
                    <a class="nav-link" :class="{ active: activeTab === 'analytics' }" @click="activeTab = 'analytics'">ANALYTICS</a>
                    <a class="nav-link" :class="{ active: activeTab === 'flamegraph' }" @click="openFlamegraphTab()">FLAMEGRAPH</a>
                </div>
            </div>
            <div class="nav-right">
                <div class="status-dot" :class="{ disconnected: !wsConnected }"></div>
                <span style="font-size: 12px; font-family: var(--font-mono); color: var(--text-secondary);">
                    {{ wsConnected ? 'CONNECTED :7787' : 'CONNECTING...' }}
                </span>
                <button class="btn-ghost" @click="refreshStatus()">REFRESH</button>
            </div>
        </nav>

        <!-- Main Container -->
        <div class="container">
            <main>
                <!-- 1. BENCHMARK RUNNER VIEW -->
                <div v-show="activeTab === 'runner'">
                    <div class="page-title">
                        <span style="width: 14px; height: 14px; background: var(--accent-lavender); border-radius: 3px; display: inline-block;"></span>
                        <span>Relay Benchmarking & Analysis</span>
                    </div>

                    <!-- 6 KPI Horizontal Cards (One Row) -->
                    <div class="kpi-grid">
                        <div class="kpi-card">
                            <div class="kpi-header">✓ COMPLETED</div>
                            <div class="kpi-value">{{ status.completed_suites_count }} / {{ status.total_suites }}</div>
                            <div class="kpi-footer">SUITES DONE</div>
                        </div>
                        <div class="kpi-card">
                            <div class="kpi-header">⚡ ACTIVE TEST</div>
                            <div class="kpi-value" style="font-size: 15px; color: var(--accent-cyan);">
                                {{ status.current_suite || (status.is_running ? 'RUNNING' : 'IDLE') }}
                            </div>
                            <div class="kpi-footer">CURRENT STEP</div>
                        </div>
                        <div class="kpi-card">
                            <div class="kpi-header">⏱ ELAPSED</div>
                            <div class="kpi-value">{{ status.elapsed_secs.toFixed(1) }}s</div>
                            <div class="kpi-footer">TOTAL TIME</div>
                        </div>
                        <div class="kpi-card">
                            <div class="kpi-header">🚀 PEAK TPS</div>
                            <div class="kpi-value" style="color: var(--accent-lime);">
                                {{ status.peak_tps > 0 ? status.peak_tps.toFixed(0) : '-' }}
                            </div>
                            <div class="kpi-footer">THROUGHPUT</div>
                        </div>
                        <div class="kpi-card">
                            <div class="kpi-header">🎯 P99 LATENCY</div>
                            <div class="kpi-value">
                                {{ status.best_p99_ms ? status.best_p99_ms.toFixed(2) + 'ms' : '-' }}
                            </div>
                            <div class="kpi-footer">BEST P99</div>
                        </div>
                        <div class="kpi-card">
                            <div class="kpi-header">💾 MEMORY RSS</div>
                            <div class="kpi-value" style="color: var(--accent-lavender);">
                                {{ status.peak_rss_mb > 0 ? status.peak_rss_mb.toFixed(1) + ' MB' : '-' }}
                            </div>
                            <div class="kpi-footer">RELAY RESIDENT</div>
                        </div>
                    </div>

                    <!-- Progress Bar Section -->
                    <div class="progress-section">
                        <div class="section-comment">
                            // live progress · {{ Math.round((status.completed_suites_count / status.total_suites) * 100) }}% completed · {{ status.current_step }}
                        </div>
                        <div class="progress-bar-bg">
                            <div class="progress-bar-fill" :style="{ width: Math.round((status.completed_suites_count / status.total_suites) * 100) + '%' }"></div>
                        </div>
                    </div>

                    <!-- Controls & Configuration Card -->
                    <div class="config-card">
                        <div class="config-header">
                            <div class="section-comment">// test configuration & execution</div>
                            <div style="display: flex; gap: 12px; flex-wrap: wrap;">
                                <!-- Target Mode Toggle -->
                                <div class="mode-toggle">
                                    <button class="mode-btn" :class="{ active: currentTargetMode === 'source' }" @click="currentTargetMode = 'source'">🏗 Build from Source</button>
                                    <button class="mode-btn" :class="{ active: currentTargetMode === 'live' }" @click="currentTargetMode = 'live'">⚡ Live Relay</button>
                                </div>

                                <!-- Test Mode Toggle (Single vs Compare) -->
                                <div class="mode-toggle" v-show="currentTargetMode === 'source'">
                                    <button class="mode-btn" :class="{ active: currentTestMode === 'single' }" @click="currentTestMode = 'single'">Single Test</button>
                                    <button class="mode-btn" :class="{ active: currentTestMode === 'compare' }" @click="currentTestMode = 'compare'">Comparison A/B</button>
                                </div>
                            </div>
                        </div>

                        <!-- Source Build Options -->
                        <div v-show="currentTargetMode === 'source'" class="controls-row">
                            <!-- Initial (Base) Branch & Commit -->
                            <div class="control-group" v-show="currentTestMode === 'compare'">
                                <label>Initial (Base Branch & Commit)</label>
                                <div style="display: flex; gap: 8px;">
                                    <select v-model="selectedBaseBranch" @change="onBaseBranchChanged()" style="width: 140px;">
                                        <option v-for="b in branches" :key="b" :value="b">{{ b }}</option>
                                    </select>
                                    <select v-model="selectedBaseCommit">
                                        <option v-for="c in baseCommits" :key="c.hash" :value="c.hash">{{ c.short_hash }} - {{ c.message.substring(0, 32) }}</option>
                                    </select>
                                </div>
                            </div>

                            <!-- Final (Target) Branch & Commit -->
                            <div class="control-group">
                                <label>{{ currentTestMode === 'compare' ? 'Final (Target Branch & Commit)' : 'Branch & Commit to Benchmark' }}</label>
                                <div style="display: flex; gap: 8px;">
                                    <select v-model="selectedTargetBranch" @change="onTargetBranchChanged()" :disabled="currentTestMode === 'compare' && compareCurrent" style="width: 140px;">
                                        <option v-for="b in branches" :key="b" :value="b">{{ b }}</option>
                                    </select>
                                    <select v-model="selectedTargetCommit" :disabled="currentTestMode === 'compare' && compareCurrent">
                                        <option v-if="currentTestMode === 'compare' && compareCurrent" value="current-codebase">(Current Codebase)</option>
                                        <option v-for="c in targetCommits" :key="c.hash" :value="c.hash">{{ c.short_hash }} - {{ c.message.substring(0, 32) }}</option>
                                    </select>
                                </div>
                            </div>

                            <div>
                                <button class="btn-lime" :disabled="status.is_running" @click="triggerRun()">
                                    <span v-if="status.is_running">⏳ RUNNING...</span>
                                    <span v-else>⚡ RUN BENCHMARK</span>
                                </button>
                            </div>
                        </div>

                        <!-- Live Testing Options -->
                        <div v-show="currentTargetMode === 'live'" class="controls-row">
                            <div class="control-group" style="grid-column: span 2;">
                                <label>Target Relay URL (ws://...)</label>
                                <input type="text" v-model="liveUrl" placeholder="ws://localhost:7777">
                            </div>
                            <div>
                                <button class="btn-lime" :disabled="status.is_running" @click="triggerRun()">
                                    <span v-if="status.is_running">⏳ TESTING...</span>
                                    <span v-else>⚡ START LIVE TEST</span>
                                </button>
                            </div>
                        </div>

                        <!-- Checkbox Toggles -->
                        <div style="display: flex; gap: 24px; margin-top: 14px; flex-wrap: wrap;">
                            <label class="checkbox-label" v-show="currentTargetMode === 'source' && currentTestMode === 'compare'">
                                <input type="checkbox" v-model="compareCurrent"> Compare Current Codebase (Stash changes)
                            </label>
                            <label class="checkbox-label" v-show="currentTargetMode === 'source'">
                                <input type="checkbox" v-model="highPerformance"> High-Performance Build (make -j$(nproc))
                            </label>
                            <label class="checkbox-label">
                                <input type="checkbox" v-model="skipHeavy"> Skip 1M Event Heavy Storage Test
                            </label>
                            <label class="checkbox-label" v-show="currentTargetMode === 'source'">
                                <input type="checkbox" v-model="flamegraph"> Generate CPU Flamegraph
                            </label>
                        </div>
                    </div>

                    <!-- Progressive Collapsible Test Suites -->
                    <div class="section-comment">// test suites & immediate results (click to expand / auto-expands on completion)</div>
                    <div class="suite-list">
                        <div v-for="(s, idx) in suitesConfig" :key="s.id" class="suite-card" :class="{ running: status.current_suite === s.id }">
                            <div class="suite-card-header" @click="toggleSuite(s.id)">
                                <div class="suite-info">
                                    <span v-if="completedMap[s.id]" class="badge-status completed">✓ COMPLETED</span>
                                    <span v-else-if="status.current_suite === s.id" class="badge-status running">⚡ TESTING</span>
                                    <span v-else class="badge-status queued">○ QUEUED</span>
                                    <span class="suite-name">{{ s.name }}</span>
                                </div>
                                <div class="suite-metrics-pill">
                                    <template v-if="completedMap[s.id]">
                                        <span class="metric-highlight">
                                            {{ completedMap[s.id].throughput ? completedMap[s.id].throughput.toFixed(0) + ' ' + (completedMap[s.id].throughput_label || 'ops/s') : '' }}
                                        </span>
                                        <span v-if="completedMap[s.id].p50_ms">P50: {{ completedMap[s.id].p50_ms.toFixed(2) }}ms</span>
                                        <span v-if="completedMap[s.id].p99_ms">P99: {{ completedMap[s.id].p99_ms.toFixed(2) }}ms</span>
                                        <span v-if="completedMap[s.id].memory_rss_mb" style="color: var(--accent-lavender);">RSS: {{ completedMap[s.id].memory_rss_mb.toFixed(1) }}MB</span>
                                        <span>{{ completedMap[s.id].elapsed_secs.toFixed(2) }}s</span>
                                    </template>
                                    <template v-else-if="status.current_suite === s.id">
                                        <span style="color: var(--accent-cyan);">Currently testing...</span>
                                    </template>
                                    <template v-else>
                                        <span style="color: var(--text-muted);">Awaiting runner...</span>
                                    </template>
                                    <span style="color: var(--text-muted); font-size: 11px;">{{ expandedSuites[s.id] ? '▲' : '▼' }}</span>
                                </div>
                            </div>

                            <!-- Collapsible Suite Body -->
                            <div v-show="expandedSuites[s.id]" class="suite-collapse-body">
                                <div v-if="completedMap[s.id]">
                                    <!-- 3-Column Stats Grid -->
                                    <div class="stats-grid">
                                        <div class="stat-box">
                                            <div class="stat-box-title">Throughput</div>
                                            <div class="stat-box-value" style="color: var(--accent-lime);">
                                                {{ completedMap[s.id].throughput ? completedMap[s.id].throughput.toFixed(1) + ' ' + (completedMap[s.id].throughput_label || 'ops/s') : '-' }}
                                            </div>
                                        </div>
                                        <div class="stat-box">
                                            <div class="stat-box-title">Latency Quantiles</div>
                                            <div class="stat-box-value" style="font-size: 12px; line-height: 1.6;">
                                                P50: {{ completedMap[s.id].p50_ms ? completedMap[s.id].p50_ms.toFixed(2) + 'ms' : '-' }} &nbsp;|&nbsp;
                                                P99: {{ completedMap[s.id].p99_ms ? completedMap[s.id].p99_ms.toFixed(2) + 'ms' : '-' }}
                                            </div>
                                        </div>
                                        <div class="stat-box">
                                            <div class="stat-box-title">Resource Footprint</div>
                                            <div class="stat-box-value" style="font-size: 12px; line-height: 1.6; color: var(--accent-lavender);">
                                                RSS: {{ completedMap[s.id].memory_rss_mb ? completedMap[s.id].memory_rss_mb.toFixed(1) + 'MB' : '-' }} &nbsp;|&nbsp;
                                                Duration: {{ completedMap[s.id].elapsed_secs.toFixed(2) }}s
                                            </div>
                                        </div>
                                    </div>

                                    <!-- Raw Output Block -->
                                    <div v-if="completedMap[s.id].log_output">
                                        <div class="section-comment" style="margin-top: 10px;">// raw benchmark output</div>
                                        <pre style="background: #07090f; border: 1px solid var(--border); border-radius: 6px; padding: 12px; font-family: var(--font-mono); font-size: 11px; color: #a0aec0; white-space: pre-wrap; line-height: 1.4; margin: 0;">{{ completedMap[s.id].log_output }}</pre>
                                    </div>
                                </div>
                                <div v-else style="color: var(--text-muted); font-size: 12px; font-family: var(--font-mono);">
                                    Suite is currently running or queued. Results will appear automatically upon completion.
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- 2. COMPARISON VIEW -->
                <div v-show="activeTab === 'comparison'">
                    <div class="page-title">
                        <span style="width: 14px; height: 14px; background: var(--accent-lime); border-radius: 3px; display: inline-block;"></span>
                        <span>A/B Benchmark Comparison</span>
                    </div>

                    <div class="config-card">
                        <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 16px;">
                            <div>
                                <div class="section-comment">// select historical comparison run</div>
                                <select v-model="selectedComparisonReportId" @change="loadComparisonReport(selectedComparisonReportId)" style="width: 360px;">
                                    <option v-for="r in pastComparisonReports" :key="r" :value="r">{{ r }}</option>
                                </select>
                            </div>
                            <div v-if="comparisonReportData" style="font-family: var(--font-mono); font-size: 12px; color: var(--text-secondary);">
                                Initial: <span style="color: var(--accent-lavender);">{{ comparisonReportData.base_ref }}</span> &nbsp;|&nbsp;
                                Final: <span style="color: var(--accent-lime);">{{ comparisonReportData.target_ref }}</span>
                            </div>
                        </div>
                    </div>

                    <div class="config-card" v-if="comparisonReportData && comparisonReportData.deltas">
                        <div class="section-comment">// performance deltas & deterministic metrics</div>
                        <table class="diff-table">
                            <thead>
                                <tr>
                                    <th>Metric</th>
                                    <th>Initial ({{ comparisonReportData.base_ref }})</th>
                                    <th>Final ({{ comparisonReportData.target_ref }})</th>
                                    <th>Delta (%)</th>
                                    <th>Status</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr v-for="d in comparisonReportData.deltas" :key="d.metric">
                                    <td style="font-weight: 600;">{{ d.metric }}</td>
                                    <td style="font-family: var(--font-mono);">{{ d.base_value.toFixed(2) }} {{ d.unit }}</td>
                                    <td style="font-family: var(--font-mono);">{{ d.target_value.toFixed(2) }} {{ d.unit }}</td>
                                    <td style="font-family: var(--font-mono); font-weight: 700;">{{ d.delta_pct > 0 ? '+' : '' }}{{ d.delta_pct.toFixed(2) }}%</td>
                                    <td><span class="delta-badge" :class="d.status.toLowerCase()">{{ d.status.toUpperCase() }}</span></td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                    <div v-else class="config-card" style="text-align: center; color: var(--text-muted); padding: 32px;">
                        No comparison report selected. Run an A/B Comparison to view deltas.
                    </div>
                </div>

                <!-- 3. PAST RUNS VIEW (Single and Comparison with Paired C1 vs C2 Layout) -->
                <div v-show="activeTab === 'past'">
                    <div class="page-title">
                        <span style="width: 14px; height: 14px; background: var(--accent-lavender); border-radius: 3px; display: inline-block;"></span>
                        <span>Historical Test Reports</span>
                    </div>

                    <div class="config-card">
                        <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 16px;">
                            <div>
                                <div class="section-comment">// select past benchmark run</div>
                                <select v-model="selectedPastReportId" @change="loadPastReport(selectedPastReportId)" style="width: 380px;">
                                    <option v-for="r in pastReports" :key="r" :value="r">
                                        {{ r.includes('compare') ? '[A/B COMPARISON] ' + r : '[SINGLE RUN] ' + r }}
                                    </option>
                                </select>
                            </div>
                            <div v-if="pastReportData" style="font-family: var(--font-mono); font-size: 12px; color: var(--text-secondary);">
                                Target: <span style="color: var(--accent-lime);">{{ pastReportData.target_ref }}</span> &nbsp;|&nbsp;
                                Date: <span style="color: var(--accent-lavender);">{{ new Date(pastReportData.timestamp).toLocaleString() }}</span>
                            </div>
                        </div>

                        <!-- Subview Navigation -->
                        <div style="display: flex; gap: 8px; margin-top: 18px; flex-wrap: wrap;" v-if="pastReportData">
                            <button class="btn-ghost" :class="{ active: pastSubView === 'paired' }" @click="pastSubView = 'paired'">
                                📑 {{ pastReportData.deltas ? 'Paired (C1 vs C2 per Suite)' : '13-Suite Breakdown' }}
                            </button>
                            <button class="btn-ghost" :class="{ active: pastSubView === 'deltas' }" v-if="pastReportData.deltas" @click="pastSubView = 'deltas'">
                                📊 Comparison Deltas ({{ pastReportData.deltas.length }})
                            </button>
                            <button class="btn-ghost" :class="{ active: pastSubView === 'markdown' }" @click="loadPastMarkdown()">
                                📝 Markdown Report
                            </button>
                            <button class="btn-ghost" :class="{ active: pastSubView === 'json' }" @click="pastSubView = 'json'">
                                💾 report.json
                            </button>
                        </div>
                    </div>

                    <!-- Subview 1: Paired C1 vs C2 Layout -->
                    <div v-show="pastSubView === 'paired'" v-if="pastReportData">
                        <!-- If Comparison Report: Paired C1 vs C2 side-by-side cards -->
                        <div v-if="pastReportData.deltas" style="display: flex; flex-direction: column; gap: 20px;">
                            <div v-for="(s, idx) in (pastReportData.target_report ? pastReportData.target_report.suites : [])" :key="s.id"
                                 class="suite-card" style="padding: 20px;">
                                <!-- Header -->
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; border-bottom: 1px solid var(--border); padding-bottom: 12px; flex-wrap: wrap; gap: 8px;">
                                    <div style="font-size: 16px; font-weight: 700; color: var(--text-primary);">
                                        <span style="color: var(--accent-lavender); margin-right: 6px;">#{{ idx + 1 }}</span> {{ s.name }}
                                    </div>
                                    <div style="font-family: var(--font-mono); font-size: 12px;">
                                        <span class="badge-status completed">COMPLETED</span>
                                    </div>
                                </div>

                                <!-- Paired Grid: Initial (C1) vs Final (C2) -->
                                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 20px;">
                                    <!-- Initial (C1) Card -->
                                    <div style="background: #090e1a; border: 1px solid rgba(138, 153, 252, 0.4); border-radius: 6px; padding: 16px;">
                                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                                            <span class="badge-status" style="background: #181d33; color: var(--accent-lavender); border: 1px solid rgba(138, 153, 252, 0.5);">
                                                Initial (C1): {{ pastReportData.base_ref }}
                                            </span>
                                            <span style="font-family: var(--font-mono); font-size: 12px; color: var(--text-muted);" v-if="getMatchingBaseSuite(s.id)">
                                                {{ getMatchingBaseSuite(s.id).elapsed_secs.toFixed(2) }}s
                                            </span>
                                        </div>
                                        <div v-if="getMatchingBaseSuite(s.id)">
                                            <div style="font-family: var(--font-mono); font-size: 13px; margin-bottom: 12px;">
                                                <span style="color: var(--accent-lavender); font-weight: 700;">
                                                    {{ getMatchingBaseSuite(s.id).throughput ? getMatchingBaseSuite(s.id).throughput.toFixed(1) + ' ' + (getMatchingBaseSuite(s.id).throughput_label || 'ops/s') : '-' }}
                                                </span>
                                                <span style="margin-left: 10px; color: var(--text-secondary);" v-if="getMatchingBaseSuite(s.id).p50_ms">P50: {{ getMatchingBaseSuite(s.id).p50_ms.toFixed(2) }}ms</span>
                                                <span style="margin-left: 8px; color: var(--text-secondary);" v-if="getMatchingBaseSuite(s.id).p99_ms">P99: {{ getMatchingBaseSuite(s.id).p99_ms.toFixed(2) }}ms</span>
                                            </div>
                                            <pre style="background: #07090f; border: 1px solid var(--border); border-radius: 6px; padding: 12px; font-family: var(--font-mono); font-size: 11px; color: #a0aec0; white-space: pre-wrap; line-height: 1.4; margin: 0;">{{ getMatchingBaseSuite(s.id).log_output }}</pre>
                                        </div>
                                        <div v-else style="color: var(--text-muted); font-size: 12px;">Not run on base commit.</div>
                                    </div>

                                    <!-- Final (C2) Card -->
                                    <div style="background: #091217; border: 1px solid rgba(210, 248, 132, 0.4); border-radius: 6px; padding: 16px;">
                                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                                            <span class="badge-status" style="background: #1c2712; color: var(--accent-lime); border: 1px solid rgba(210, 248, 132, 0.5);">
                                                Final (C2): {{ pastReportData.target_ref }}
                                            </span>
                                            <span style="font-family: var(--font-mono); font-size: 12px; color: var(--text-muted);">
                                                {{ s.elapsed_secs.toFixed(2) }}s
                                            </span>
                                        </div>
                                        <div style="font-family: var(--font-mono); font-size: 13px; margin-bottom: 12px;">
                                            <span style="color: var(--accent-lime); font-weight: 700;">
                                                {{ s.throughput ? s.throughput.toFixed(1) + ' ' + (s.throughput_label || 'ops/s') : '-' }}
                                            </span>
                                            <span style="margin-left: 10px; color: var(--text-secondary);" v-if="s.p50_ms">P50: {{ s.p50_ms.toFixed(2) }}ms</span>
                                            <span style="margin-left: 8px; color: var(--text-secondary);" v-if="s.p99_ms">P99: {{ s.p99_ms.toFixed(2) }}ms</span>
                                        </div>
                                        <pre style="background: #07090f; border: 1px solid var(--border); border-radius: 6px; padding: 12px; font-family: var(--font-mono); font-size: 11px; color: #a0aec0; white-space: pre-wrap; line-height: 1.4; margin: 0;">{{ s.log_output }}</pre>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <!-- If Single Run Report: Clean 13 Suite Cards -->
                        <div v-else style="display: flex; flex-direction: column; gap: 16px;">
                            <div v-for="(s, idx) in pastReportData.suites" :key="s.id" class="suite-card" style="padding: 18px;">
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 14px; flex-wrap: wrap; gap: 8px;">
                                    <div style="display: flex; align-items: center; gap: 12px;">
                                        <span class="badge-status completed">✓ #{{ idx + 1 }} {{ s.name }}</span>
                                        <span style="font-size: 12px; color: var(--text-muted); font-family: var(--font-mono);">{{ s.elapsed_secs.toFixed(2) }}s</span>
                                    </div>
                                    <div style="font-family: var(--font-mono); font-size: 12px;">
                                        <span class="metric-highlight">{{ s.throughput ? s.throughput.toFixed(1) + ' ' + (s.throughput_label || 'ops/s') : '-' }}</span>
                                        <span v-if="s.p50_ms" style="color: var(--text-secondary); margin-left: 12px;">P50: {{ s.p50_ms.toFixed(2) }}ms</span>
                                        <span v-if="s.p99_ms" style="color: var(--text-secondary); margin-left: 8px;">P99: {{ s.p99_ms.toFixed(2) }}ms</span>
                                        <span v-if="s.memory_rss_mb" style="color: var(--accent-lavender); margin-left: 10px;">RSS: {{ s.memory_rss_mb.toFixed(1) }}MB</span>
                                    </div>
                                </div>
                                <pre style="background: #07090f; border: 1px solid var(--border); border-radius: 6px; padding: 12px; font-family: var(--font-mono); font-size: 11px; color: #a0aec0; white-space: pre-wrap; line-height: 1.4; margin: 0;">{{ s.log_output }}</pre>
                            </div>
                        </div>
                    </div>

                    <!-- Subview 2: Deltas Table -->
                    <div v-show="pastSubView === 'deltas'" v-if="pastReportData && pastReportData.deltas" class="config-card">
                        <table class="diff-table">
                            <thead>
                                <tr>
                                    <th>Metric</th>
                                    <th>Initial ({{ pastReportData.base_ref }})</th>
                                    <th>Final ({{ pastReportData.target_ref }})</th>
                                    <th>Delta (%)</th>
                                    <th>Status</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr v-for="d in pastReportData.deltas" :key="d.metric">
                                    <td style="font-weight: 600;">{{ d.metric }}</td>
                                    <td style="font-family: var(--font-mono);">{{ d.base_value.toFixed(2) }} {{ d.unit }}</td>
                                    <td style="font-family: var(--font-mono);">{{ d.target_value.toFixed(2) }} {{ d.unit }}</td>
                                    <td style="font-family: var(--font-mono); font-weight: 700;">{{ d.delta_pct > 0 ? '+' : '' }}{{ d.delta_pct.toFixed(2) }}%</td>
                                    <td><span class="delta-badge" :class="d.status.toLowerCase()">{{ d.status.toUpperCase() }}</span></td>
                                </tr>
                            </tbody>
                        </table>
                    </div>

                    <!-- Subview 3: Markdown Report View -->
                    <div v-show="pastSubView === 'markdown'" class="config-card">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                            <span class="section-comment">// GitHub-Flavored Markdown Report</span>
                            <button class="btn-ghost" @click="copyToClipboard(pastMarkdownText)">📋 Copy Markdown</button>
                        </div>
                        <pre style="background: #07090f; padding: 16px; border-radius: 6px; border: 1px solid var(--border); font-family: var(--font-mono); font-size: 12px; color: #a0aec0; white-space: pre-wrap; line-height: 1.5;">{{ pastMarkdownText }}</pre>
                    </div>

                    <!-- Subview 4: Raw JSON View -->
                    <div v-show="pastSubView === 'json'" class="config-card">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                            <span class="section-comment">// Full Machine-Readable JSON</span>
                            <button class="btn-ghost" @click="copyToClipboard(JSON.stringify(pastReportData, null, 2))">📋 Copy JSON</button>
                        </div>
                        <pre style="background: #07090f; padding: 16px; border-radius: 6px; border: 1px solid var(--border); font-family: var(--font-mono); font-size: 12px; color: #a0aec0; white-space: pre-wrap; max-height: 600px; overflow-y: auto;">{{ JSON.stringify(pastReportData, null, 2) }}</pre>
                    </div>
                </div>

                <!-- 4. ANALYTICS VIEW -->
                <div v-show="activeTab === 'analytics'">
                    <div class="page-title">
                        <span style="width: 14px; height: 14px; background: var(--accent-cyan); border-radius: 3px; display: inline-block;"></span>
                        <span>Throughput & Latency Distribution</span>
                    </div>

                    <div class="config-card">
                        <div class="section-comment">// suite throughput comparison (events/sec & req/sec)</div>
                        <div style="display: flex; flex-direction: column; gap: 14px; margin-top: 16px;">
                            <div v-for="s in analyticsSuites" :key="s.name">
                                <div style="display: flex; justify-content: space-between; font-size: 13px; margin-bottom: 4px;">
                                    <span>{{ s.name }}</span>
                                    <span style="font-family: var(--font-mono); color: var(--accent-lime);">
                                        {{ s.throughput.toFixed(0) }} {{ s.throughput_label || 'ops/s' }}
                                    </span>
                                </div>
                                <div style="width: 100%; height: 10px; background-color: var(--bg-canvas); border-radius: 5px; overflow: hidden; border: 1px solid var(--border);">
                                    <div :style="{ width: Math.max(5, (s.throughput / maxAnalyticsTps) * 100) + '%' }" style="height: 100%; background-color: var(--accent-lime);"></div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- 5. FLAMEGRAPH VIEW -->
                <div v-show="activeTab === 'flamegraph'">
                    <div class="page-title">
                        <span style="width: 14px; height: 14px; background: #ff9800; border-radius: 3px; display: inline-block;"></span>
                        <span>CPU Flamegraph Inspection</span>
                    </div>

                    <div class="flamegraph-viewer">
                        <div class="flamegraph-toolbar">
                            <div style="display: flex; align-items: center; gap: 12px; flex-wrap: wrap;">
                                <span class="section-comment" style="margin: 0;">// benchmark run</span>
                                <select v-model="selectedFlamegraphReportId" @change="onFlamegraphReportChanged()" style="width: 280px; padding: 6px 10px; font-size: 12px;">
                                    <option v-for="r in pastReports" :key="r" :value="r">{{ r }}</option>
                                </select>

                                <span class="section-comment" style="margin: 0;">// available svg</span>
                                <select v-model="selectedFlamegraphFile" @change="onFlamegraphFileChanged()" style="width: 240px; padding: 6px 10px; font-size: 12px;">
                                    <option v-for="f in flamegraphFiles" :key="f.file" :value="f.file">{{ f.label }}</option>
                                </select>
                            </div>
                            <div style="display: flex; gap: 8px;">
                                <button class="btn-ghost" @click="refreshFlamegraph()">⟳ REFRESH</button>
                                <a v-if="selectedFlamegraphFile" class="btn-ghost" style="text-decoration: none;" :href="flamegraphUrl" target="_blank" download>⤓ DOWNLOAD SVG</a>
                            </div>
                        </div>
                        <div v-if="flamegraphFiles.length === 0" style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 500px; text-align: center; padding: 40px;">
                            <div style="font-size: 36px; margin-bottom: 12px;">🔥</div>
                            <div style="font-size: 18px; font-weight: 700; color: var(--text-primary); margin-bottom: 8px;">No Flamegraph Found in This Run</div>
                            <div style="font-size: 13px; color: var(--text-secondary); max-width: 520px; margin-bottom: 24px; line-height: 1.6;">
                                CPU profiling was not enabled for this benchmark run. Make sure <strong>"Generate CPU Flamegraph"</strong> is checked in the Benchmark tab, or pass <code>--flamegraph</code> on the CLI.
                            </div>
                            <button class="btn-lime" @click="activeTab = 'runner'">⚡ Go to Benchmark Runner</button>
                        </div>
                        <iframe v-show="flamegraphFiles.length > 0" class="flamegraph-frame" :src="flamegraphUrl"></iframe>
                    </div>
                </div>
            </main>

            <!-- Right Sidebar (Telemetry & Live Log Stream) -->
            <aside class="sidebar">
                <div class="sidebar-card">
                    <div class="section-comment">// context & live telemetry</div>
                    <div style="font-size: 13px; font-weight: 600; margin-bottom: 12px; color: var(--accent-cyan);">
                        {{ status.current_step }}
                    </div>
                    <div class="log-terminal" id="log-terminal">
                        <div v-for="(line, i) in logs" :key="i">{{ line }}</div>
                    </div>
                </div>
            </aside>
        </div>

        <!-- Bottom Status Bar -->
        <footer class="footer">
            <div>
                <span class="brand-badge" style="font-size: 10px; padding: 2px 6px;">STRFRY</span>
                <span>{{ status.is_running ? 'Benchmark Active · Running' : 'Ready' }}</span>
            </div>
            <div>~/strfry-bench/report</div>
        </footer>
    </div>

    <script>
        const { createApp } = Vue;

        createApp({
            data() {
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

                    logs: ['Connecting to live benchmark stream...'],
                    wsConnected: false
                };
            },
            computed: {
                flamegraphUrl() {
                    if (!this.selectedFlamegraphReportId || !this.selectedFlamegraphFile) return 'about:blank';
                    return `/api/reports/${encodeURIComponent(this.selectedFlamegraphReportId)}/${encodeURIComponent(this.selectedFlamegraphFile)}`;
                },
                analyticsSuites() {
                    const suites = Object.values(this.completedMap).filter(s => s && s.throughput);
                    if (suites.length > 0) return suites;
                    if (this.pastReportData) {
                        return (this.pastReportData.suites || (this.pastReportData.target_report && this.pastReportData.target_report.suites) || []).filter(s => s && s.throughput);
                    }
                    return [];
                },
                maxAnalyticsTps() {
                    const tpsArr = this.analyticsSuites.map(s => s.throughput);
                    return tpsArr.length > 0 ? Math.max(...tpsArr) : 1000;
                }
            },
            methods: {
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
                            else if (f === 'final_flamegraph.svg') label = 'Final Flamegraph (Target)';
                            else if (f === 'initial_flamegraph.svg') label = 'Initial Flamegraph (Base)';
                            this.flamegraphFiles.push({ file: f, label });
                        });

                        if (this.flamegraphFiles.length > 0) {
                            this.selectedFlamegraphFile = this.flamegraphFiles[0].file;
                        } else {
                            this.selectedFlamegraphFile = '';
                        }
                    } catch (e) {
                        console.error("Failed to load flamegraphs", e);
                    }
                },
                onFlamegraphFileChanged() {
                    // updates computed flamegraphUrl
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
                            this.$nextTick(() => {
                                const term = document.getElementById('log-terminal');
                                if (term) term.scrollTop = term.scrollHeight;
                            });
                        }
                    };

                    ws.onclose = () => {
                        this.wsConnected = false;
                        setTimeout(() => this.connectWs(), 2000);
                    };
                }
            },
            mounted() {
                this.fetchBranches();
                this.loadReportsList();
                this.refreshStatus();
                this.connectWs();
            }
        }).mount('#app');
    </script>
</body>
</html>
"##;
