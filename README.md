# 1 Billion Row Challenge in Rust

| Version | Description                              | Time (s) | Throughput  | Improvement |
| ------- | ---------------------------------------- | -------- | ----------- | ----------- |
| v1      | Single-threaded                          | 133.361  | 98.65 MB/s  | -           |
| v2      | Multi-threaded. Whole file loaded to RAM | 39.758   | 330.91 MB/s | x3.35       |
| v3      | v2 + reduce allocations in hot loop      | 17.863   | 736.51 MB/s | x2.23       |

## Run and compare

Data downloaded from: https://huggingface.co/datasets/nietras/1brc.data/tree/main

One billion rows:

```sh
./run.sh 1000000000
```

Small dataset:

```sh
./run.sh 1000000
```
