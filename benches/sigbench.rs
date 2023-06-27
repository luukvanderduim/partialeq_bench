use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

#[derive(Debug, Clone, Copy)]
struct D<'a, 'b>(&'a str, &'b str);

impl<'a, 'b> From<D<'a, 'b>> for (&'a str, &'b str) {
    fn from(d: D<'a, 'b>) -> Self {
        let D(sig_a, sig_b) = d;
        (sig_a, sig_b)
    }
}

impl<'a, 'b> std::fmt::Display for D<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let D(sig_a, sig_b) = self;
        write!(f, "({sig_a}, {sig_b})")
    }
}

impl<'a, 'b> From<(&'a str, &'b str)> for D<'a, 'b> {
    fn from(sigpair: (&'a str, &'b str)) -> Self {
        D(sigpair.0, sigpair.1)
    }
}

static LONG_PAIR: (&str, &str) = ("soy(ba{v})soy(ba{v})bbba{v}", "soy(ba{v})soy(ba{v})bbba{v}");
static LONG_PAIR_NEQ: (&str, &str) = (
    "soy(ba{v})soy(ba{v})bbba{v}",
    "soy(ba{v})soy(ba{v})bbba{v}a",
);
static MEDIUM_PAIR: (&str, &str) = ("siia{vo}(ss)(o)", "(siia{vo}(ss)(o))");
static SHORT_PAIR: (&str, &str) = ("ii", "(ii)");

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

pub fn bench_eq(c: &mut Criterion) {
    let mut group = c.benchmark_group("Single Signatures");
    let data = [SHORT_PAIR, MEDIUM_PAIR, LONG_PAIR, LONG_PAIR_NEQ];

    for pair in data.iter().map(|pair| D::from(*pair)) {
        group.bench_with_input(BenchmarkId::new("pre-PR", pair), &pair, |b, pair| {
            let (str_a, str_b) = (*pair).into();
            b.iter(|| {
                black_box(str_a == str_b);
            });
        });

        group.bench_with_input(BenchmarkId::new("as_bytes", pair), &pair, |b, pair| {
            let (str_a, str_b) = (*pair).into();
            b.iter(|| {
                black_box(is_equal_bytes((str_a, str_b)));
            });
        });

        group.bench_with_input(BenchmarkId::new("as_char", pair), &pair, |b, pair| {
            let (str_a, str_b) = (*pair).into();
            b.iter(|| {
                black_box(is_equal_chars((str_a, str_b)));
            });
        });

        group.bench_with_input(BenchmarkId::new("sliced_str", pair), &pair, |b, pair| {
            let (str_a, str_b) = (*pair).into();
            b.iter(|| {
                black_box(is_equal_sliced_str((str_a, str_b)));
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_eq);
criterion_main!(benches);
