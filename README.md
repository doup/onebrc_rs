# 1 Billion Row Challenge in Rust

| Version | Description                              | Time: 5 run avg. (s) | Throughput  | Improvement |
| ------- | ---------------------------------------- | -------------------- | ----------- | ----------- |
| v1      | Single-threaded                          | 133.361              | 98.65 MB/s  | -           |
| v2      | Multi-threaded. Whole file loaded to RAM | 39.758               | 330.91 MB/s | x3.35       |
| v3      | v2 + reduce allocations in hot loop      | 17.863               | 736.51 MB/s | x2.23       |

## Run and check result

Data downloaded from: https://huggingface.co/datasets/nietras/1brc.data/tree/main

One billion rows:

```sh
./run.sh 1000000000
```

Small dataset:

```sh
./run.sh 1000000
```

## Profiling

Profile with [samply](https://github.com/mstange/samply):

```sh
cargo build --profile profiling && samply record ./target/profiling/onebrc_rs 100000
```
