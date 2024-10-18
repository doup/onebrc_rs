# 1 Billion Row Challenge in Rust

| Version | Description                              | Time (s) | Improvement |
| ------- | ---------------------------------------- | -------- | ----------- |
| v1      | Single-threaded                          | 133.361  | -           |
| v2      | Multi-threaded. Whole file loaded to RAM | 39.758   | x3.35       |

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
