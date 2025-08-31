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

# Security and Safety

The pseudo-random algorithms in here are not cryptographically secure.
They are not even unbiased on a simple level.

The checksum implementations have not been audited by a third-party
for use in safety critical applications.  While they provide some test
coverage, the test sources have not been checked themselves.

This crate can provide a good platform to learn about checksums and
coding theory.  By experimenting with and visualizing the objects in
different ways I have found it easier myself to better understand
them.


## Visualizations

To see an example feedback shift register diagram of a CRC:

cargo run --example shift_register_diagram


# Contributing

Efficiency wasn't a driving factor with this project.  It was mainly
meant to explore checksums and hashes and understand them.

If you would like to optimize the code or have bug fixes,
contributions are welcome.


# References

Phil Koopman's YouTube example that covers polynomial division and
shift register representations:

[L600 CRC Computation Examples -- Polynomial Division & Hardware Shift Register](https://www.youtube.com/watch?v=1t3DacyL5HA)

Implementing CRCs by Jack Crenshaw from Embedded Systems Programming, 1992
[Implementing CRCs](https://www.scribd.com/document/617993743/JCrenshaw-ImplementingCRCs)
