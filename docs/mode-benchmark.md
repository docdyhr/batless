| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/batless benchmark_files/medium.rs --mode=plain` | 2.2 ± 0.1 | 2.0 | 3.2 | 1.15 ± 0.09 |
| `./target/release/batless benchmark_files/medium.rs --mode=highlight` | 2.2 ± 0.4 | 2.0 | 14.8 | 1.16 ± 0.21 |
| `./target/release/batless benchmark_files/medium.rs --mode=json` | 2.0 ± 0.1 | 1.8 | 3.2 | 1.03 ± 0.09 |
| `./target/release/batless benchmark_files/medium.rs --mode=summary` | 1.9 ± 0.1 | 1.7 | 2.8 | 1.00 |
