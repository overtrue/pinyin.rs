# Performance Notes

The current matcher uses longest-prefix dictionary lookup:

1. Load generated dictionary data once with `OnceLock`.
2. At each input position, try the longest possible dictionary key first.
3. Fall back to the original character when no entry matches.

This avoids the previous full automaton startup cost and keeps first-use latency practical for CLI usage.

Current local smoke results:

- `cargo test`: completes in a few seconds on the local macOS workspace.
- `pinyin "你好世界"` returns immediately after normal debug-build startup.
- `cargo clippy --all-targets --all-features -- -D warnings`: passes.

Run Criterion benchmarks when you need publishable numbers:

```bash
cargo bench
```
