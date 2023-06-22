# PartialEq bench

In a PR for zvariant, which alters `Signature` equality to evaluate irrespective of outer parentheses,
we ended up with two different approaches for matching or selecting and subslicing on the signature string.

## 'Pre-PR' condition

As reference, the 'pre-PR' condition, I chose the simple `&str` equality test.

```rust
sig_a == sig_b
```

Note, that all tests differ at the first character, which might make this even more efficient.

One manipulating 'chars':

```rust
fn without_outer_parentheses_chars(sig: &str) -> Option<&str> {
    if sig.starts_with('(') && sig.ends_with(')') {
        let subslice = sig.get(1..sig.len() - 1).unwrap();

        if subslice.chars().fold(0, |count, ch| match ch {
            '(' => count + 1,
            ')' if count != 0 => count - 1,
            _ => count,
        }) == 0
        {
            return Some(subslice);
        }
    };

    None
}
```

And the 'bytes' variant:

```rust
fn without_outer_parentheses_bytes(sig: &str) -> Option<&[u8]> {
    if let [b'(', subslice @ .., b')'] = sig.as_bytes() {
        if subslice.iter().fold(0, |count, ch| match ch {
            b'(' => count + 1,
            b')' if count != 0 => count - 1,
            _ => count,
        }) == 0
        {
            return Some(subslice);
        }
    };

    None
}
```

I assumed bytewise would be faster than char-wise.
But was that even true, and if so, by what troubling amount?

The PR aims to address unexpected inequality of `Signature`s describing the same type or 'type code block'.
It does so at the cost of a slightly more expensive `PartialEq`.

How expensive would `==` become in either case?

## Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
  - [Single Signatures](#single-signatures)

# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
  - [Single Signatures](#single-signatures)

## Benchmark Results

### Single Signatures

|                                                                 | `pre-PR`                | `as_bytes`                      | `as_char`                         |
|:----------------------------------------------------------------|:------------------------|:--------------------------------|:--------------------------------- |
| **`ii, (ii)`**                                                  | `0.22 ns` (✅ **1.00x**) | `5.61 ns` (❌ *26.02x slower*)   | `8.00 ns` (❌ *37.08x slower*)     |
| **`siia{vo}(ss)(o), (siia{vo}(ss)(o))`**                        | `0.22 ns` (✅ **1.00x**) | `9.95 ns` (❌ *46.14x slower*)   | `17.06 ns` (❌ *79.11x slower*)    |
| **`soy(ba{v})soy(ba{v})bbba{v}, soy(ba{v})soy(ba{v})bbba{v}`**  | `2.16 ns` (✅ **1.00x**) | `4.10 ns` (❌ *1.90x slower*)    | `6.05 ns` (❌ *2.80x slower*)      |
| **`soy(ba{v})soy(ba{v})bbba{v}, soy(ba{v})soy(ba{v})bbba{v}a`** | `0.22 ns` (✅ **1.00x**) | `2.59 ns` (❌ *12.02x slower*)   | `4.11 ns` (❌ *19.06x slower*)     |
---

Note 1: We are comparing apples to pears because the 'pre-PR' can bail out on the first character on the first and second pair.
Nevertheless, this is just about what it does before (if) the PR lands.

In the third pair and the last pair, this effect becomes apparent because, while the third pair is byte-wise equal, the last pair deviates at the very last chance.
This dramatically improves chances for the smarter `PartialEq` implementations to catch up, because neither  

These benches ran on an AMD Ryzen 7 5700.

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)
