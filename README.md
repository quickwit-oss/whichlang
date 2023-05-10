# Whichlang

This is a language detection library, aiming for both precision and performance.

# Features

- No dependency
- Throughput above 100 MB/s for short and long strings.
- Good accuracy (99.5% on my validation dataset, but it really depends on the size of your input.)

# How does it work?

It uses a multiclass logistic regression model over:
- 2, 3, 4-grams of letters on ASCII
- codepoint / 128
- a slightly smarter projection of codepoints over a given class.

We use the hashing trick and project these features over a space of size `4_096`.

The logistic regression is trained in the python notebook attached,
and used to generate `weight.rs`.

# Comparison with [Whatlang](https://github.com/greyblake/whatlang-rs)


The following compares the throughput using the simple benchmark found in this repository and compares the accuracy using [whatlang-corpora](https://github.com/whatlang/whatlang-corpora) benchmark. Overall, Whichlang is about 50x faster and slightly more accurate than Whatlang. 

### Throughput

|                           | Processing Time (µs) | Throughput (MiB/s) |
| ------------------------- | -------------------- | ------------------ | 
| whatlang/short            | 56.229               | 0.48               | 
| whatlang/long             | 142.1                | 4.1141             | 
| whichlang/short           | 0.261                | 105.69             | 
| whichlang/long            | 5.205                | 112.31             | 

### Accuracy

|            | Size <= 20 | 21-50  | 51-100 | Size > 100 | AVG   |
| -----------| ---------- | ------ | ------ | ---------- | ----- |
| Whatlang   | 62.58%     | 81.84% | 92.95% | 97.13%     |83.63% |
| Whichlang  | 92.10%     | 97.55% | 99.06% | 99.39%     |97.03% |
