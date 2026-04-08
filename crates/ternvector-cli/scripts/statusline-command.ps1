# TernVector Intelligence Statusline for Windows PowerShell
# Multi-line display showcasing self-learning capabilities

$ErrorActionPreference = "SilentlyContinue"

# Read JSON input from stdin
$input = [Console]::In.ReadToEnd()
$data = $input | ConvertFrom-Json

$Model = if ($data.model.display_name) { $data.model.display_name } else { "Claude" }
$CWD = if ($data.workspace.current_dir) { $data.workspace.current_dir } else { $data.cwd }
$Dir = Split-Path -Leaf $CWD

# Get git branch
$Branch = $null
Push-Location $CWD 2>$null
$Branch = git branch --show-current 2>$null
Pop-Location

# ANSI colors (Windows Terminal supports these)
$Reset = "`e[0m"
$Bold = "`e[1m"
$Cyan = "`e[36m"
$Yellow = "`e[33m"
$Green = "`e[32m"
$Magenta = "`e[35m"
$Blue = "`e[34m"
$Red = "`e[31m"
$Dim = "`e[2m"

# ═══════════════════════════════════════════════════════════════════════════════
# LINE 1: Model, Directory, Git
# ═══════════════════════════════════════════════════════════════════════════════
$Line1 = "${Bold}${Model}${Reset} in ${Cyan}${Dir}${Reset}"
if ($Branch) {
    $Line1 += " on ${Yellow}⎇ ${Branch}${Reset}"
}
Write-Host $Line1

# ═══════════════════════════════════════════════════════════════════════════════
# LINE 2: TernVector Intelligence Stats
# ═══════════════════════════════════════════════════════════════════════════════
$IntelFile = $null
$IntelPaths = @(
    "$CWD\.ruvector\intelligence.json",
    "$CWD\npm\packages\ruvector\.ruvector\intelligence.json",
    "$env:USERPROFILE\.ruvector\intelligence.json"
)

foreach ($path in $IntelPaths) {
    if (Test-Path $path) {
        $IntelFile = $path
        break
    }
}

if ($IntelFile) {
    $Intel = Get-Content $IntelFile -Raw | ConvertFrom-Json

    # Detect schema version
    $HasLearning = $Intel.PSObject.Properties.Name -contains "learning"

    if ($HasLearning) {
        # v2 Schema
        $PatternCount = 0
        if ($Intel.learning.qTables) {
            foreach ($table in $Intel.learning.qTables.PSObject.Properties) {
                $PatternCount += $table.Value.PSObject.Properties.Count
            }
        }

        $ActiveAlgos = 0
        $TotalAlgos = 0
        $BestAlgo = "none"
        $BestScore = 0

        if ($Intel.learning.stats) {
            $stats = $Intel.learning.stats.PSObject.Properties
            $TotalAlgos = $stats.Count
            foreach ($stat in $stats) {
                if ($stat.Value.updates -gt 0) {
                    $ActiveAlgos++
                    if ($stat.Value.convergenceScore -gt $BestScore) {
                        $BestScore = $stat.Value.convergenceScore
                        $BestAlgo = $stat.Name
                    }
                }
            }
        }

        $RoutingAlgo = if ($Intel.learning.configs.'agent-routing'.algorithm) {
            $Intel.learning.configs.'agent-routing'.algorithm
        } else { "double-q" }
        $LearningRate = if ($Intel.learning.configs.'agent-routing'.learningRate) {
            $Intel.learning.configs.'agent-routing'.learningRate
        } else { 0.1 }
        $Epsilon = if ($Intel.learning.configs.'agent-routing'.epsilon) {
            $Intel.learning.configs.'agent-routing'.epsilon
        } else { 0.1 }
        $Schema = "v2"
    }
    else {
        # v1 Schema
        $PatternCount = if ($Intel.patterns) { $Intel.patterns.PSObject.Properties.Count } else { 0 }
        $TrajCount = if ($Intel.trajectories) { $Intel.trajectories.Count } else { 0 }
        $ActiveAlgos = 0
        $TotalAlgos = 0
        $BestAlgo = "none"
        $BestScore = 0
        $RoutingAlgo = "q-learning"
        $LearningRate = 0.1
        $Epsilon = 0.1
        $Schema = "v1"
    }

    # Common fields
    $MemoryCount = if ($Intel.memories) { $Intel.memories.Count } else { 0 }
    $TrajCount = if ($Intel.trajectories) { $Intel.trajectories.Count } else { 0 }
    $ErrorCount = if ($Intel.errors) { $Intel.errors.Count } else { 0 }
    $SessionCount = if ($Intel.stats.session_count) { $Intel.stats.session_count } else { 0 }

    # Build Line 2
    $Line2 = "${Magenta}🧠 TernVector${Reset}"

    if ($PatternCount -gt 0) {
        $Line2 += " ${Green}◆${Reset} $PatternCount patterns"
    } else {
        $Line2 += " ${Dim}◇ learning${Reset}"
    }

    if ($ActiveAlgos -gt 0) {
        $Line2 += " ${Cyan}⚙${Reset} $ActiveAlgos/$TotalAlgos algos"
    }

    if ($BestAlgo -ne "none") {
        $ShortAlgo = switch ($BestAlgo) {
            "double-q" { "DQ" }
            "q-learning" { "QL" }
            "actor-critic" { "AC" }
            "decision-transformer" { "DT" }
            "monte-carlo" { "MC" }
            "td-lambda" { "TD" }
            default { $BestAlgo.Substring(0,3) }
        }
        $ScorePct = [math]::Round($BestScore * 100)
        $ScoreColor = if ($ScorePct -ge 80) { $Green } elseif ($ScorePct -ge 50) { $Yellow } else { $Red }
        $Line2 += " ${ScoreColor}★${ShortAlgo}:${ScorePct}%${Reset}"
    }

    if ($MemoryCount -gt 0) {
        $Line2 += " ${Blue}⬡${Reset} $MemoryCount mem"
    }

    if ($TrajCount -gt 0) {
        $Line2 += " ${Yellow}↝${Reset}$TrajCount"
    }

    if ($ErrorCount -gt 0) {
        $Line2 += " ${Red}🔧${Reset}$ErrorCount"
    }

    if ($SessionCount -gt 0) {
        $Line2 += " ${Dim}#$SessionCount${Reset}"
    }

    Write-Host $Line2

    # ═══════════════════════════════════════════════════════════════════════════════
    # LINE 3: Agent Routing
    # ═══════════════════════════════════════════════════════════════════════════════
    $AlgoIcon = switch ($RoutingAlgo) {
        "double-q" { "⚡DQ" }
        "sarsa" { "🔄SA" }
        "actor-critic" { "🎭AC" }
        default { $RoutingAlgo }
    }

    $LrPct = [math]::Round($LearningRate * 100)
    $EpsPct = [math]::Round($Epsilon * 100)

    $Line3 = "${Blue}🎯 Routing${Reset} ${Cyan}${AlgoIcon}${Reset} lr:${LrPct}% ε:${EpsPct}%"

    Write-Host $Line3
}
else {
    Write-Host "${Dim}🧠 TernVector: run 'npx ruvector hooks session-start' to initialize${Reset}"
}

# ═══════════════════════════════════════════════════════════════════════════════
# LINE 4: Claude Flow (if available)
# ═══════════════════════════════════════════════════════════════════════════════
$FlowDir = "$CWD\.claude-flow"
if (Test-Path $FlowDir) {
    $FlowOutput = ""

    $SwarmConfig = "$FlowDir\swarm-config.json"
    if (Test-Path $SwarmConfig) {
        $Config = Get-Content $SwarmConfig -Raw | ConvertFrom-Json
        if ($Config.defaultStrategy) {
            $Topo = switch ($Config.defaultStrategy) {
                "balanced" { "mesh" }
                "conservative" { "hier" }
                "aggressive" { "ring" }
                default { $Config.defaultStrategy }
            }
            $FlowOutput += " ${Magenta}${Topo}${Reset}"
        }
        if ($Config.agentProfiles -and $Config.agentProfiles.Count -gt 0) {
            $FlowOutput += " ${Cyan}🤖$($Config.agentProfiles.Count)${Reset}"
        }
    }

    if ($FlowOutput) {
        Write-Host "${Dim}⚡ Flow:${Reset}$FlowOutput"
    }
}
