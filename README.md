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
