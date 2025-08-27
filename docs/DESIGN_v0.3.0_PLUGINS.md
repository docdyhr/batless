
# v0.3.0 Plugin Architecture Design

## Overview

Enable extensible analyzers, formatters, and integrations while maintaining batless's core guarantees: never block, always stream, bounded resources.

## Core Requirements

### Must Have

- **Safety**: Plugins cannot crash batless or cause hangs
- **Streaming**: Plugins must process data incrementally
- **Performance**: Plugin overhead < 10% for typical use
- **Simplicity**: < 100 lines for a basic plugin

### Nice to Have

- Hot reload without restart
- Cross-platform plugin distribution
- Plugin marketplace/registry

## Architecture Decision: WASM-First

After evaluating options, WASM provides the best balance:

| Criteria | Native DLL | WASM | JSON-RPC |
|----------|------------|------|----------|
| Safety | ❌ Can crash host | ✅ Sandboxed | ✅ Process isolation |
| Performance | ✅ Native speed | ✅ Near-native | ❌ IPC overhead |
| Cross-platform | ❌ Platform builds | ✅ Universal | ✅ Universal |
| Complexity | Medium | Low | High |
| **Decision** | No | **Yes** | Fallback |

## Plugin Interface

```rust
// Plugin trait (exposed via WASM)
pub trait BatlessPlugin {
    /// Plugin metadata
    fn info(&self) -> PluginInfo;

    /// Process a chunk of lines (streaming)
    fn process_chunk(&mut self, lines: &[String]) -> Result<ProcessedChunk>;

    /// Finalize and return any remaining output
    fn finalize(&mut self) -> Result<Option<String>>;
}

pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub supported_languages: Vec<String>,
    pub output_format: OutputFormat,
}

pub struct ProcessedChunk {
    pub output: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub continue_processing: bool,
}
```

## Implementation Phases

### Phase 1: Core System (Week 1-2)

- [x] Research wasmtime vs wasmer (choose wasmtime for stability)
- [ ] Define plugin trait and WASM ABI
- [ ] Implement plugin loader
- [ ] Add plugin discovery (~/.batless/plugins/)

### Phase 2: Plugin SDK (Week 3)

- [ ] Create `batless-plugin` crate
- [ ] Add helper macros for plugin creation
- [ ] Create plugin template repository
- [ ] Write plugin development guide

### Phase 3: Example Plugins (Week 4)

- [ ] TODO/FIXME finder plugin
- [ ] Statistics plugin (LOC, complexity)
- [ ] Markdown formatter plugin
- [ ] CSV output plugin

## Example Plugin (TODO Finder)

```rust
use batless_plugin::*;

#[derive(Default)]
struct TodoFinder {
    todos: Vec<Todo>,
}

#[plugin]
impl BatlessPlugin for TodoFinder {
    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "todo-finder".into(),
            version: "0.1.0".into(),
            description: "Find TODO/FIXME comments".into(),
            supported_languages: vec!["*".into()],
            output_format: OutputFormat::Json,
        }
    }

    fn process_chunk(&mut self, lines: &[String]) -> Result<ProcessedChunk> {
        for (idx, line) in lines.iter().enumerate() {
            if line.contains("TODO") || line.contains("FIXME") {
                self.todos.push(Todo {
                    line_number: idx,
                    content: line.clone(),
                    priority: detect_priority(line),
                });
            }
        }

        Ok(ProcessedChunk {
            output: None, // Buffer until finalize
            metadata: None,
            continue_processing: true,
        })
    }

    fn finalize(&mut self) -> Result<Option<String>> {
        Ok(Some(serde_json::to_string(&self.todos)?))
    }
}
```

## Performance Budget

- Plugin load time: < 5ms
- Per-chunk overhead: < 1ms for 100 lines
- Memory overhead: < 10MB per plugin
- Total overhead with 5 plugins: < 10% of baseline

## Security Model

- Plugins run in WASM sandbox
- No filesystem access by default
- No network access
- Memory limited to 50MB per plugin
- CPU time limited to 100ms per chunk

## Success Metrics

- [ ] Load and execute example plugin
- [ ] Process 1MB file with < 10% overhead
- [ ] Plugin crashes don't affect batless
- [ ] 3 community plugins within first month
