# tapestry-analysis
![build](https://github.com/jgerrish/tapestry-analysis/actions/workflows/rust.yml/badge.svg)

Collection of cryptanalysis tools for checksum and hash algorithms.

# Introduction

These tools arose out of some simple experiments with Adler32,
Fletcher and CRC checksums.  I needed a checksum solution for
resource-constrained devices that was still effective.

Existing public documentation indicated Adler32 had issues with small
data sizes.  To evaluate the checksum, I created a simple terminal
histogram example.  This was followed by Kolmogorov-Smirnov test for
testing whether the distribution of checksums was uniform.

Additional code may be added to perform Chi-Square tests on data,
compute Hamming distance and other common tests.

# Usage

The example adler32_output_space_use shows an example of using
histograms and statistical tests to visualize and test distributions.

cargo run --example adler32_output_space_use


This uses the CRC32 library from checksum-tapestry, if you want to use
the rand crate, enable it with a feature:

cargo run --example adler32_output_space_use --features external-rand


# Contributing

Efficiency wasn't a driving factor with this project.  It was mainly
meant to explore checksums and hashes and understand them.

If you would like to optimize the code or have bug fixes,
contributions are welcome.
