use criterion::{black_box, criterion_group, criterion_main, Criterion};

// A list of 10 random DBus signature string pairs of different lengths, with and without outer parentheses.
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

fn char_wise(c: &mut Criterion) {
    c.bench_function("with chars", |b| {
        b.iter(|| {
            for sigpair in SIGNATURES {
                black_box(is_equal_chars(*sigpair));
            }
        })
    });
}

fn byte_wise(c: &mut Criterion) {
    c.bench_function("with bytes", |b| {
        b.iter(|| {
            for sigpair in SIGNATURES {
                black_box(is_equal_bytes(*sigpair));
            }
        })
    });
}

criterion_group!(benches, char_wise, byte_wise);
criterion_main!(benches);
