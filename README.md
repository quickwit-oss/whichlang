# Whichlang

This is a language detection library, aiming for both precision and performance. You can read our [blog post](https://quickwit.io/blog/whichlang-language-detection-library) that introduces the algorithm behing Whichlang.

# Features

- No dependency
- Throughput above 100 MB/s for short and long strings.
- Good accuracy (99.5% on my validation dataset, but it really depends on the size of your input.)
- Supported languages: Arabic, Dutch, English, French, German, Hindi, Italian, Japanese, Korean, Mandarin, Portuguese, Russian, Spanish, Swedish, Turkish, and Vietnamese.

# How does it work?

It uses a multiclass logistic regression model over:
- 2, 3, 4-grams of letters on ASCII
- codepoint / 128
- a slightly smarter projection of codepoints over a given class.

We use the hashing trick and project these features over a space of size `4_096`.

The logistic regression is trained in the python notebook attached,
and used to generate `weight.rs`.

# Comparison with [Whatlang](https://github.com/greyblake/whatlang-rs)

The following compares the throughput using the simple benchmark found in this repository and the accuracy using [whatlang-accuracy-benchmark](https://github.com/evanxg852000/whatlang-accuracy-benchmark) benchmark. Overall, Whichlang is about 10x faster and slightly more accurate than Whatlang.

### Throughput

To generate the throughput benchmark, we ported the benchmark available in [this repository](https://github.com/quickwit-oss/whichlang/blob/main/benches/bench.rs). Please, check this [repository](https://github.com/evanxg852000/whatlang-accuracy-benchmark) to see our changes.

|                           | Processing Time (µs) | Throughput (MiB/s) |
| ------------------------- | -------------------- | ------------------ | 
| whatlang/short            | 16.62                | 1.66               | 
| whatlang/long             | 62.00                | 9.42               | 
| whichlang/short           | 0.26                 | 105.69             | 
| whichlang/long            | 5.21                 | 112.31             | 

### Accuracy


To generate the accuracy benchmark, we have changed the [whatlang-accuracy-benchmark](https://github.com/whatlang/whatlang-accuracy-benchmark) to add support for Whichlang. Given that Whatlang supports more languages, we have used its FilterList feature to restrict its analysis to only languages that are supported in Whichlang. We also use the `trigram` method in Whatlang.  Please, check this [repository](https://github.com/evanxg852000/whatlang-accuracy-benchmark) to see our changes.

```
Crate: Whatlang
AVG: 91.69%

| LANG       | AVG    | <= 20   | 21-50  | 51-100 | > 100   |
|------------|--------|---------|--------|--------|---------|
| Arabic     | 99.68% | 99.51%  | 99.64% | 99.83% | 99.76%  |
| Mandarin   | 96.09% | 97.54%  | 96.92% | 95.45% | 94.43%  |
| German     | 88.57% | 70.00%  | 88.53% | 96.61% | 99.16%  |
| English    | 85.99% | 57.82%  | 88.37% | 97.97% | 99.78%  |
| French     | 90.88% | 72.84%  | 92.51% | 98.54% | 99.65%  |
| Hindi      | 99.80% | 100.00% | 99.83% | 99.78% | 99.61%  |
| Italian    | 87.75% | 66.67%  | 87.74% | 97.04% | 99.54%  |
| Japanese   | 94.37% | 93.97%  | 96.04% | 94.30% | 93.18%  |
| Korean     | 99.17% | 98.88%  | 99.69% | 99.44% | 98.66%  |
| Dutch      | 89.68% | 72.13%  | 89.78% | 97.40% | 99.40%  |
| Portuguese | 88.08% | 72.90%  | 85.76% | 95.22% | 98.44%  |
| Russian    | 99.98% | 100.00% | 99.96% | 99.98% | 100.00% |
| Spanish    | 82.91% | 55.45%  | 82.24% | 94.85% | 99.10%  |
| Swedish    | 84.16% | 58.33%  | 83.78% | 96.35% | 98.18%  |
| Turkish    | 86.73% | 61.01%  | 88.94% | 97.32% | 99.63%  |
| Vietnamese | 93.23% | 82.84%  | 92.96% | 97.88% | 99.24%  |
| AVG        | 91.69% | 78.74%  | 92.04% | 97.37% | 98.61%  |
```

```
Crate: Whichlang
AVG: 97.03%

| LANG       | AVG     | <= 20   | 21-50   | 51-100  | > 100   |
|------------|---------|---------|---------|---------|---------|
| Arabic     | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| Mandarin   | 98.65%  | 98.69%  | 98.48%  | 98.55%  | 98.87%  |
| German     | 94.20%  | 80.00%  | 97.47%  | 99.49%  | 99.84%  |
| English    | 97.15%  | 91.84%  | 97.25%  | 99.57%  | 99.93%  |
| French     | 97.59%  | 93.83%  | 97.61%  | 99.20%  | 99.71%  |
| Hindi      | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| Italian    | 97.20%  | 93.06%  | 97.33%  | 98.85%  | 99.57%  |
| Japanese   | 94.92%  | 88.95%  | 95.14%  | 97.74%  | 97.85%  |
| Korean     | 99.83%  | 99.44%  | 99.98%  | 99.97%  | 99.94%  |
| Dutch      | 97.08%  | 92.84%  | 96.98%  | 98.91%  | 99.60%  |
| Portuguese | 94.07%  | 83.87%  | 94.89%  | 98.18%  | 99.36%  |
| Russian    | 99.92%  | 99.69%  | 99.99%  | 100.00% | 100.00% |
| Spanish    | 92.12%  | 76.36%  | 93.78%  | 98.65%  | 99.70%  |
| Swedish    | 95.37%  | 90.28%  | 94.94%  | 97.76%  | 98.51%  |
| Turkish    | 95.51%  | 88.24%  | 98.11%  | 98.38%  | 97.33%  |
| Vietnamese | 98.79%  | 96.57%  | 98.87%  | 99.77%  | 99.96%  |
| AVG        | 97.03%  | 92.10%  | 97.55%  | 99.06%  | 99.39%  |
```
