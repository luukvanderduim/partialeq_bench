# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Single Signatures](#single-signatures)

## Benchmark Results

### Single Signatures

|                                                                   | `pre-PR`                | `as_bytes`                      | `as_char`                         |
|:------------------------------------------------------------------|:------------------------|:--------------------------------|:--------------------------------- |
| **`(ii, (ii))`**                                                  | `0.22 ns` (✅ **1.00x**) | `5.61 ns` (❌ *26.02x slower*)   | `8.00 ns` (❌ *37.08x slower*)     |
| **`(siia{vo}(ss)(o), (siia{vo}(ss)(o)))`**                        | `0.22 ns` (✅ **1.00x**) | `9.95 ns` (❌ *46.14x slower*)   | `17.06 ns` (❌ *79.11x slower*)    |
| **`(soy(ba{v})soy(ba{v})bbba{v}, soy(ba{v})soy(ba{v})bbba{v})`**  | `2.16 ns` (✅ **1.00x**) | `4.10 ns` (❌ *1.90x slower*)    | `6.05 ns` (❌ *2.80x slower*)      |
| **`(soy(ba{v})soy(ba{v})bbba{v}, soy(ba{v})soy(ba{v})bbba{v}a)`** | `0.22 ns` (✅ **1.00x**) | `2.59 ns` (❌ *12.02x slower*)   | `4.11 ns` (❌ *19.06x slower*)     |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

