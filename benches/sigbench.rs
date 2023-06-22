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

pub(crate) fn without_outer_parentheses(sig: &str) -> Option<&[u8]> {
    if sig_str.starts_with('(') && sig_str.ends_with(')') {
        let subslice = self.slice(1..self.len() - 1);

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
pub(crate) fn without_outer_parentheses_bytes(sig: &str) -> Option<&str> {
    let sig_str = self.as_str();

    if sig_str.starts_with('(') && sig_str.ends_with(')') {
        let subslice = self.slice(1..self.len() - 1);

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

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
