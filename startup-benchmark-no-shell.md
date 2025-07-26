| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/batless benchmark_files/small.rs` | 1.6 ± 0.1 | 1.4 | 2.7 | 1.00 |
| `./target/release/batless benchmark_files/medium.rs` | 2.2 ± 0.1 | 2.0 | 3.0 | 1.35 ± 0.13 |
| `./target/release/batless benchmark_files/large.rs --max-lines=100` | 1.9 ± 0.1 | 1.7 | 2.6 | 1.17 ± 0.12 |
