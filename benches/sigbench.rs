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

static SIGNATURES: &[(&str, &str)] = &[
    ("ii", "(ii)"),
    ("so(ii)", "(so(ii))"),
    ("(so)(so)(so)", "((so)(so)(so))"),
    ("(so)yb(so)", "((so)yb(so))"),
    ("sss(o)", "(sss(o))"),
    ("(((o)))", "(o)"),
    ("((o))", "(o)"),
    ("siiva{si}so", "(siiva{si}so)"),
    ("siiva{si}so", "siiva{si}so"),
    (
        "siiva{si}sosiiva{si}sosiiva{si}so",
        "(siiva{si}sosiiva{si}sosiiva{si}so)",
    ),
    ("susuassusau(o)", "susuassussau(o)"),
    ("susuassusau(o)", "(susuassussau(o))"),
    ("su(s)((u))assusau(o)", "(su(s)((u))assussau(o))"),
    (
        "soyba{v}soyba{v}soyba{v}soyba{v}",
        "(soyba{v}soyba{v}soyba{v}soyba{v})",
    ),
    (
        "soy(ba{v})soy(ba{v})soyba{v}soyba{v}",
        "(soy(ba{v})soy(ba{v})soyba{v}soyba{v})",
    ),
    ("f", "s"),
    ("(f)", "(s)"),
    (")", "("),
    (")(", ")("),
    (")))(((", "()()()"),
    ("sous", "(sous)"),
    ("sousb", "(sousb)"),
    ("sousba", "(sousba)"),
    ("sousbas", "(sousbas)"),
    ("sousbass", "(sousbass)"),
    ("sousbasss", "(sousbasss)"),
    ("sousbassss", "(sousbassss)"),
    ("sousbasssss", "(sousbasssss)"),
];

static COMPLEX_PAIR: (&str, &str) = (
    "soy(ba{v})soy(ba{v})soyba{v}soyba{v}",
    "(soy(ba{v})soy(ba{v})soyba{v}soyba{v})",
);

static MEDIUM_PAIR: (&str, &str) = ("susuassusau(o)", "(susuassussau(o))");

static SIMPLE_PAIR: (&str, &str) = ("ii", "(ii)");

pub(crate) fn without_outer_parentheses_chars(sig: &str) -> Option<&str> {
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

pub(crate) fn without_outer_parentheses_bytes(sig: &str) -> Option<&[u8]> {
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

pub fn bench_eq(c: &mut Criterion) {
    let mut group = c.benchmark_group("Single Signatures");
    let data = vec![SIMPLE_PAIR, MEDIUM_PAIR, COMPLEX_PAIR];
    for pair in data.iter().map(|pair| D::from(*pair)) {
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
    }
    group.finish();
}

fn bench_eq_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("List of Signatures");
    let pairs = Vec::from(SIGNATURES);

    for pair in pairs.iter().map(|pair| D::from(*pair)) {
        group.bench_with_input(BenchmarkId::new("as_bytes", pair), &pair, |b, pair| {
            let (str_a, str_b) = (*pair).into();
            b.iter(|| {
                black_box(is_equal_bytes((str_a, str_b)));
            });
        });

        group.bench_with_input(BenchmarkId::new("as_chars", pair), &pair, |b, pair| {
            b.iter(|| {
                let (str_a, str_b) = (*pair).into();
                black_box(is_equal_chars((str_a, str_b)));
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_eq, bench_eq_list);
criterion_main!(benches);
