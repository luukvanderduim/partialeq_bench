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

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
