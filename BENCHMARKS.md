# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
  - [Single Signatures](#single-signatures)

## Benchmark Results

### Single Signatures

|                                                                     | `pre-PR`                | `as_bytes`                        | `as_char`                         | `sliced_str`                       |
|:--------------------------------------------------------------------|:------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`( ii, (ii) )`**                                                  | `0.22 ns` (✅ **1.00x**) | `5.90 ns` (❌ *27.37x slower*)    | `8.09 ns` (❌ *37.51x slower*)    | `6.27 ns` (❌ *29.10x slower*)     |
| **`( siia{vo}(ss)(o), (siia{vo}(ss)(o)) )`**                        | `0.22 ns` (✅ **1.00x**) | `10.21 ns` (❌ *47.36x slower*)   | `17.14 ns` (❌ *79.51x slower*)   | `11.96 ns` (❌ *55.51x slower*)    |
| **`( soy(ba{v})soy(ba{v})bbba{v}, soy(ba{v})soy(ba{v})bbba{v} )`**  | `2.16 ns` (✅ **1.00x**) | `4.32 ns` (❌ *2.00x slower*)     | `5.85 ns` (❌ *2.71x slower*)     | `4.75 ns` (❌ *2.20x slower*)      |
| **`( soy(ba{v})soy(ba{v})bbba{v}, soy(ba{v})soy(ba{v})bbba{v}a )`** | `0.22 ns` (✅ **1.00x**) | `2.59 ns` (❌ *11.96x slower*)    | `4.12 ns` (❌ *19.04x slower*)    | `3.02 ns` (❌ *13.94x slower*)     |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)
