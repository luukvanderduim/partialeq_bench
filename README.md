# PartialEq bench

In a PR for zvariant, which alters `Signature` equality to evaluate irrespective of outer parentheses,
we ended up with a few approaches for matching or selecting and subslicing on the signature string.

I assumed bytewise would be faster than char-wise, the zvariant author suggested the `sliced_str` variant.

The PR aims to address unexpected inequality of `Signature`s describing the same type or 'type code block'.
It does so at the cost of a slightly more expensive `PartialEq`.

How expensive would `==` become in each case?

## Benchmarks

### Table of Contents

- [Benchmark Results](#benchmark-results)
  - [Single Signatures](#single-signatures)

### Benchmark Results

#### Single Signatures

|                                                                     | `pre-PR`                | `as_bytes`                        | `as_char`                         | `sliced_str`                       |
|:--------------------------------------------------------------------|:------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`( ii, (ii) )`**                                                  | `0.22 ns` (✅ **1.00x**) | `5.90 ns` (❌ *27.37x slower*)    | `8.09 ns` (❌ *37.51x slower*)    | `6.27 ns` (❌ *29.10x slower*)     |
| **`( siia{vo}(ss)(o), (siia{vo}(ss)(o)) )`**                        | `0.22 ns` (✅ **1.00x**) | `10.21 ns` (❌ *47.36x slower*)   | `17.14 ns` (❌ *79.51x slower*)   | `11.96 ns` (❌ *55.51x slower*)    |
| **`( soy(ba{v})soy(ba{v})bbba{v}, soy(ba{v})soy(ba{v})bbba{v} )`**  | `2.16 ns` (✅ **1.00x**) | `4.32 ns` (❌ *2.00x slower*)     | `5.85 ns` (❌ *2.71x slower*)     | `4.75 ns` (❌ *2.20x slower*)      |
| **`( soy(ba{v})soy(ba{v})bbba{v}, soy(ba{v})soy(ba{v})bbba{v}a )`** | `0.22 ns` (✅ **1.00x**) | `2.59 ns` (❌ *11.96x slower*)    | `4.12 ns` (❌ *19.04x slower*)    | `3.02 ns` (❌ *13.94x slower*)     |

---

These benches ran on an AMD Ryzen 7 5700G.

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

## benched functions

These functions approximate what is used in situ.

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

fn without_outer_parentheses_bytes_sliced_str(sig: &str) -> &str {
    let sig_str = sig;

    if let Some(subslice) = sig_str.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
        if subslice.chars().fold(0, |count, ch| match ch {
            '(' => count + 1,
            ')' if count != 0 => count - 1,
            _ => count,
        }) == 0
        {
            return subslice;
        }
    }

    sig_str
}

fn is_equal_chars(sigpair: (&str, &str)) -> bool {
    let (sig_a, sig_b) = sigpair;
    match (
        without_outer_parentheses_chars(sig_a),
        without_outer_parentheses_chars(sig_b),
    ) {
        (Some(subslice), None) => subslice == sig_b,
        (None, Some(subslice)) => subslice == sig_a,
        _ => sig_a == sig_b,
    }
}

fn is_equal_bytes(sigpair: (&str, &str)) -> bool {
    let (sig_a, sig_b) = sigpair;
    match (
        without_outer_parentheses_bytes(sig_a),
        without_outer_parentheses_bytes(sig_b),
    ) {
        (Some(subslice), None) => subslice == sig_b.as_bytes(),
        (None, Some(subslice)) => subslice == sig_a.as_bytes(),
        _ => sig_a == sig_b,
    }
}

fn is_equal_sliced_str(sigpair: (&str, &str)) -> bool {
    let (sig_a, sig_b) = sigpair;
    without_outer_parentheses_bytes_sliced_str(sig_a)
        == without_outer_parentheses_bytes_sliced_str(sig_b)
}

```
