| Command | Mean [µs] | Min [µs] | Max [µs] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/batless benchmark_files/small.rs` | 561.6 ± 449.5 | 0.0 | 2178.7 | 1.00 |
| `./target/release/batless benchmark_files/medium.rs` | 1281.3 ± 631.4 | 695.1 | 9717.3 | 2.28 ± 2.14 |
| `./target/release/batless benchmark_files/large.rs --max-lines=100` | 963.4 ± 376.3 | 466.6 | 2591.3 | 1.72 ± 1.53 |
