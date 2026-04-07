# TernVector Hooks Architecture

Technical architecture documentation for the TernVector hooks system.

## Table of Contents

1. [System Overview](#system-overview)
2. [Component Architecture](#component-architecture)
3. [Intelligence Layer](#intelligence-layer)
4. [Hook Execution Flow](#hook-execution-flow)
5. [Data Storage](#data-storage)
6. [Integration Points](#integration-points)
7. [Security Model](#security-model)
8. [Performance Optimization](#performance-optimization)

---

## System Overview

### High-Level Architecture

```
┌──────────────────────────────────────────────────────────────────────┐
│                         Claude Code Runtime                           │
├──────────────────────────────────────────────────────────────────────┤
│                                                                        │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐             │
│  │  PreToolUse │────►│  Tool Exec  │────►│ PostToolUse │             │
│  │    Hooks    │     │             │     │    Hooks    │             │
│  └──────┬──────┘     └─────────────┘     └──────┬──────┘             │
│         │                                        │                    │
│         ▼                                        ▼                    │
│  ┌────────────────────────────────────────────────────────────┐      │
│  │                    Hook Dispatcher                          │      │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │      │
│  │  │ Matcher  │  │ Executor │  │ Response │  │ Timeout  │   │      │
│  │  │  Engine  │  │  Engine  │  │ Handler  │  │ Manager  │   │      │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │      │
│  └────────────────────────────────────────────────────────────┘      │
│                              │                                        │
│                              ▼                                        │
│  ┌────────────────────────────────────────────────────────────┐      │
│  │                  TernVector CLI Layer                         │      │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │      │
│  │  │ Command  │  │ Template │  │   Path   │  │ Config   │   │      │
│  │  │ Parser   │  │  Engine  │  │ Resolver │  │ Manager  │   │      │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │      │
│  └────────────────────────────────────────────────────────────┘      │
│                              │                                        │
│                              ▼                                        │
│  ┌────────────────────────────────────────────────────────────┐      │
│  │                   Intelligence Layer                        │      │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │      │
│  │  │Q-Learning│  │  Vector  │  │  Agent   │  │  Neural  │   │      │
│  │  │  Engine  │  │  Memory  │  │  Router  │  │ Trainer  │   │      │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │      │
│  └────────────────────────────────────────────────────────────┘      │
│                              │                                        │
│                              ▼                                        │
│  ┌────────────────────────────────────────────────────────────┐      │
│  │                    Storage Layer                            │      │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐                 │      │
│  │  │   JSON   │  │  RvLite  │  │  Global  │                 │      │
│  │  │  Files   │  │   HNSW   │  │ Patterns │                 │      │
│  │  └──────────┘  └──────────┘  └──────────┘                 │      │
│  └────────────────────────────────────────────────────────────┘      │
│                                                                        │
└──────────────────────────────────────────────────────────────────────┘
```

### Design Principles

1. **Portability**: No hardcoded paths; runtime resolution
2. **Minimal Overhead**: <100ms total hook overhead
3. **Graceful Degradation**: Hooks never block main flow
4. **Learning by Default**: Automatic pattern improvement
5. **Cross-Platform**: Linux, macOS, Windows support

---

## Component Architecture

### CLI Layer (`crates/ruvector-cli`)

The command-line interface for hook management.

```
crates/ruvector-cli/
├── src/
│   ├── cli/
│   │   ├── commands.rs        # Command definitions
│   │   ├── hooks/             # Hooks subcommands
│   │   │   ├── mod.rs         # Module exports
│   │   │   ├── init.rs        # hooks init
│   │   │   ├── install.rs     # hooks install
│   │   │   ├── migrate.rs     # hooks migrate
│   │   │   ├── stats.rs       # hooks stats
│   │   │   ├── export.rs      # hooks export
│   │   │   └── import.rs      # hooks import
│   │   └── ...
│   └── main.rs
├── templates/                  # Hook templates
│   ├── hooks.json.j2          # Portable hooks template
│   ├── config.toml.j2         # Configuration template
│   └── gitignore.j2           # Gitignore template
└── Cargo.toml
```

### Template Engine

Uses Askama for type-safe template rendering:

```rust
#[derive(Template)]
#[template(path = "hooks.json.j2")]
struct HookTemplate {
    shell: String,          // Platform shell wrapper
    ruvector_cli: String,   // CLI invocation method
    project_root: String,   // Project root path
}

fn render_hooks() -> Result<String> {
    let template = HookTemplate {
        shell: get_shell_wrapper(),
        ruvector_cli: get_cli_invocation(),
        project_root: env::current_dir()?.display().to_string(),
    };
    Ok(template.render()?)
}
```

### Path Resolution

Dynamic path resolution at runtime:

```rust
pub fn get_ruvector_home() -> Result<PathBuf> {
    // Priority order:
    // 1. RUVECTOR_HOME environment variable
    // 2. ~/.ruvector (Unix) or %APPDATA%\ruvector (Windows)

    if let Ok(home) = env::var("RUVECTOR_HOME") {
        return Ok(PathBuf::from(shellexpand::tilde(&home).to_string()));
    }

    let home_dir = dirs::home_dir()
        .ok_or_else(|| anyhow!("Could not determine home directory"))?;

    Ok(home_dir.join(".ruvector"))
}

pub fn get_cli_path() -> Result<String> {
    // Priority order:
    // 1. Binary in PATH
    // 2. npx ruvector
    // 3. Current executable

    if let Ok(path) = which::which("ruvector") {
        return Ok(path.display().to_string());
    }

    Ok("npx ruvector".to_string())
}
```

---

## Intelligence Layer

### Q-Learning Engine

Implements temporal difference learning for action selection:

```javascript
// Q-value update equation
// Q(s,a) ← Q(s,a) + α[r + γ max_a' Q(s',a') - Q(s,a)]

class QLearning {
    constructor(options = {}) {
        this.alpha = options.learningRate || 0.1;   // Learning rate
        this.gamma = options.discount || 0.95;      // Discount factor
        this.qTable = new Map();                    // State-action values
    }

    update(state, action, reward, nextState) {
        const currentQ = this.getQ(state, action);
        const maxNextQ = this.getMaxQ(nextState);
        const newQ = currentQ + this.alpha * (reward + this.gamma * maxNextQ - currentQ);
        this.setQ(state, action, newQ);
    }

    selectAction(state) {
        // Epsilon-greedy exploration
        if (Math.random() < this.epsilon) {
            return this.randomAction(state);
        }
        return this.bestAction(state);
    }
}
```

### State Representation

States encode file context and action type:

```javascript
function encodeState(context) {
    const { file, crate, tool, previousSuccess } = context;

    return {
        fileType: getFileExtension(file),           // 'rs', 'ts', 'py'
        crateName: crate || 'unknown',              // 'ruvector-core'
        toolCategory: categorize(tool),             // 'edit', 'bash', 'search'
        historyHash: hashRecent(previousSuccess),   // Recent success pattern
    };
}

// State key for Q-table lookup
function stateKey(state) {
    return `${state.toolCategory}_${state.fileType}_in_${state.crateName}`;
}
```

### Vector Memory

Semantic search using HNSW indexing:

```javascript
class VectorMemory {
    constructor(dimensions = 128) {
        this.dimensions = dimensions;
        this.index = new HnswIndex({ dimensions, maxElements: 50000 });
        this.metadata = new Map();
    }

    async store(key, text, metadata) {
        const embedding = await this.embed(text);
        const id = this.index.add(embedding);
        this.metadata.set(id, { key, ...metadata });
        return id;
    }

    async search(query, k = 5) {
        const embedding = await this.embed(query);
        const results = this.index.search(embedding, k);
        return results.map(r => ({
            ...this.metadata.get(r.id),
            distance: r.distance
        }));
    }

    async embed(text) {
        // Simple embedding: TF-IDF + dimensionality reduction
        // Production: Use sentence-transformers or similar
        return textToEmbedding(text, this.dimensions);
    }
}
```

### Agent Router

Intelligent agent assignment based on context:

```javascript
class AgentRouter {
    constructor(qLearning, vectorMemory) {
        this.q = qLearning;
        this.memory = vectorMemory;
        this.agentTypes = loadAgentTypes();
    }

    async route(context) {
        const state = encodeState(context);

        // 1. Check Q-learning suggestion
        const qSuggestion = this.q.selectAction(state);
        const qConfidence = this.q.getQ(state, qSuggestion);

        // 2. Check similar past edits
        const similar = await this.memory.search(context.file, 3);
        const historyAgent = this.majorityVote(similar);

        // 3. Apply file type heuristics
        const heuristicAgent = this.fileTypeHeuristic(context.file);

        // 4. Combine signals
        return this.combine({
            q: { agent: qSuggestion, confidence: qConfidence },
            history: { agent: historyAgent, confidence: similar.length / 3 },
            heuristic: { agent: heuristicAgent, confidence: 0.5 }
        });
    }

    fileTypeHeuristic(file) {
        const ext = path.extname(file);
        const mapping = {
            '.rs': 'rust-developer',
            '.ts': 'typescript-developer',
            '.tsx': 'react-developer',
            '.py': 'python-developer',
            '.go': 'go-developer',
            '.sql': 'database-specialist',
        };
        return mapping[ext] || 'coder';
    }
}
```

---

## Hook Execution Flow

### PreToolUse Flow

```
┌─────────────────┐
│  Claude Code    │
│  Tool Request   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Match Hooks    │  ──► No match ──► Execute Tool
│  (Regex/Type)   │
└────────┬────────┘
         │ Match
         ▼
┌─────────────────┐
│  Execute Hook   │  timeout: 3000ms
│  Command        │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Parse Result   │
│  (JSON/stdout)  │
└────────┬────────┘
         │
         ▼
    ┌────┴────┐
    │continue?│
    └────┬────┘
    │    │
   Yes   No
    │    │
    ▼    ▼
Execute  Block
 Tool    Tool
```

### PostToolUse Flow

```
┌─────────────────┐
│  Tool Completed │
│  (Result Ready) │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Match Hooks    │
│  (Regex/Type)   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Execute Hook   │  (async, non-blocking)
│  Command        │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Intelligence   │
│  Update         │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Q-value Train  │
│  Memory Store   │
│  Pattern Update │
└─────────────────┘
```

### Session Hook Flow

```
Session Start                           Session End
     │                                       │
     ▼                                       ▼
┌──────────┐                          ┌──────────┐
│  Load    │                          │  Persist │
│  Config  │                          │  State   │
└────┬─────┘                          └────┬─────┘
     │                                     │
     ▼                                     ▼
┌──────────┐                          ┌──────────┐
│ Restore  │                          │  Export  │
│ Memory   │                          │ Metrics  │
└────┬─────┘                          └────┬─────┘
     │                                     │
     ▼                                     ▼
┌──────────┐                          ┌──────────┐
│ Display  │                          │ Generate │
│ Status   │                          │ Summary  │
└──────────┘                          └──────────┘
```

---

## Data Storage

### Directory Structure

```
Project Root
├── .ruvector/                    # Project-local data
│   ├── config.toml               # Configuration
│   ├── intelligence/
│   │   ├── memory.json           # Vector memory (JSON fallback)
│   │   ├── patterns.json         # Q-learning patterns
│   │   ├── trajectories.json     # Learning history
│   │   ├── feedback.json         # User feedback
│   │   └── memory.rvdb           # RvLite vector database
│   ├── logs/
│   │   └── hooks-YYYY-MM-DD.log  # Daily logs
│   └── .gitignore
│
└── .claude/
    └── settings.json             # Hook configurations

Global (~/.ruvector/)
├── global/
│   ├── patterns.json             # Cross-project patterns
│   ├── memory.rvdb               # Global vector memory
│   └── sequences.json            # Common file sequences
├── config.toml                   # Global configuration
└── cache/
    └── cli-path.txt              # Cached CLI location
```

### Data Formats

#### patterns.json (Q-Learning)

```json
{
  "edit_rs_in_ruvector-core": {
    "rust-developer": {
      "q_value": 0.823,
      "update_count": 47,
      "last_update": "2025-12-27T10:30:00Z"
    },
    "coder": {
      "q_value": 0.312,
      "update_count": 5,
      "last_update": "2025-12-20T14:22:00Z"
    }
  }
}
```

#### trajectories.json (Learning History)

```json
[
  {
    "id": "traj_001",
    "state": "edit_rs_in_ruvector-core",
    "action": "rust-developer",
    "outcome": "success",
    "reward": 1.0,
    "timestamp": "2025-12-27T10:30:00Z",
    "ab_group": "treatment",
    "metadata": {
      "file": "crates/ruvector-core/src/lib.rs",
      "duration_ms": 1500
    }
  }
]
```

#### memory.rvdb (Vector Database)

RvLite database with HNSW indexing:

```sql
-- Schema (auto-created)
CREATE TABLE memories (
    id TEXT PRIMARY KEY,
    embedding VECTOR(128),
    content TEXT,
    metadata JSON,
    created_at TIMESTAMP
);

CREATE INDEX memories_hnsw ON memories USING hnsw (embedding);
```

---

## Integration Points

### Claude Code Integration

Hook configuration in `.claude/settings.json`:

```json
{
  "hooks": {
    "PreToolUse": [{
      "matcher": "pattern",    // Regex for tool name
      "hooks": [{
        "type": "command",     // Shell command
        "command": "...",      // Command to execute
        "timeout": 3000        // Timeout in ms
      }]
    }],
    "PostToolUse": [...],
    "SessionStart": [...],
    "Stop": [...]              // Session end
  }
}
```

### Claude-Flow Integration

MCP tool coordination:

```javascript
// Pre-task hook with MCP
async function preTask(description) {
    // Store in coordination memory
    await mcp__claude_flow__memory_usage({
        action: "store",
        key: "swarm/task/current",
        namespace: "coordination",
        value: JSON.stringify({ description, started: Date.now() })
    });

    // Spawn recommended agents
    const agents = analyzeTaskNeeds(description);
    for (const agent of agents) {
        await mcp__claude_flow__agent_spawn({ type: agent });
    }
}
```

### Git Integration

Pre-commit hook example:

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Run TernVector pre-edit on staged files
FILES=$(git diff --cached --name-only --diff-filter=ACM)

for FILE in $FILES; do
    RESULT=$(npx ruvector hooks pre-edit --file "$FILE" --validate-syntax)
    CONTINUE=$(echo "$RESULT" | jq -r '.continue')

    if [ "$CONTINUE" = "false" ]; then
        echo "Pre-edit hook blocked: $FILE"
        echo "$RESULT" | jq -r '.reason'
        exit 1
    fi
done
```

---

## Security Model

### Command Injection Prevention

All user inputs are escaped:

```rust
use shell_escape::escape;

fn generate_hook_command(file_path: &str) -> String {
    let escaped = escape(file_path.into());
    format!(
        r#"/bin/bash -c 'npx ruvector hooks pre-edit --file {}'"#,
        escaped
    )
}

// Prevents: "; rm -rf /" attacks
// file_path = "test.ts; rm -rf /"
// escaped = "'test.ts; rm -rf /'"  (treated as literal string)
```

### Timeout Protection

Hooks cannot hang indefinitely:

```javascript
async function executeHookSafely(hook, timeout = 3000) {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), timeout);

    try {
        const result = await executeHook(hook, { signal: controller.signal });
        clearTimeout(timeoutId);
        return result;
    } catch (error) {
        if (error.name === 'AbortError') {
            console.warn('Hook timeout, continuing...');
            return { continue: true, reason: 'timeout' };
        }
        throw error;
    }
}
```

### Graceful Failure

Hooks never block tool execution:

```javascript
async function runPreHook(tool, context) {
    try {
        const result = await executeHookSafely(hook);
        return result.continue !== false;
    } catch (error) {
        console.warn(`Hook failed: ${error.message}`);
        return true;  // Continue on error
    }
}
```

---

## Performance Optimization

### Caching Strategy

```javascript
class IntelligenceCache {
    constructor(maxAge = 300000) {  // 5 minutes
        this.cache = new Map();
        this.maxAge = maxAge;
    }

    get(key) {
        const entry = this.cache.get(key);
        if (!entry) return null;
        if (Date.now() - entry.timestamp > this.maxAge) {
            this.cache.delete(key);
            return null;
        }
        return entry.value;
    }

    set(key, value) {
        this.cache.set(key, { value, timestamp: Date.now() });
    }
}

// Cache agent routing decisions
const routingCache = new IntelligenceCache();

async function getAgent(file) {
    const cached = routingCache.get(file);
    if (cached) return cached;

    const agent = await computeAgent(file);
    routingCache.set(file, agent);
    return agent;
}
```

### Async Operations

Non-blocking post-hooks:

```javascript
// Fire-and-forget for training
function postEditHook(file, success) {
    // Synchronous: quick response
    const response = { continue: true, formatted: true };

    // Async: training (non-blocking)
    setImmediate(() => {
        trainPatterns(file, success).catch(console.warn);
        updateMemory(file).catch(console.warn);
        recordTrajectory(file, success).catch(console.warn);
    });

    return response;
}
```

### Batch Operations

Reduce I/O with batching:

```javascript
class BatchWriter {
    constructor(flushInterval = 5000) {
        this.queue = [];
        this.interval = setInterval(() => this.flush(), flushInterval);
    }

    add(item) {
        this.queue.push(item);
    }

    async flush() {
        if (this.queue.length === 0) return;

        const batch = this.queue.splice(0);
        await fs.appendFile(
            'trajectories.json',
            batch.map(JSON.stringify).join('\n') + '\n'
        );
    }
}

const trajectoryWriter = new BatchWriter();
```

### Performance Targets

| Operation | Target | Typical |
|-----------|--------|---------|
| Pre-edit hook | <50ms | 30ms |
| Post-edit hook | <100ms | 60ms |
| Session start | <200ms | 150ms |
| Memory search | <10ms | 5ms |
| Q-value lookup | <1ms | 0.1ms |
| Total overhead | <100ms | 70ms |

---

## See Also

- [User Guide](USER_GUIDE.md) - Getting started
- [CLI Reference](CLI_REFERENCE.md) - Command documentation
- [Migration Guide](MIGRATION.md) - Upgrade from other systems
- [Implementation Plan](IMPLEMENTATION_PLAN.md) - Development roadmap
