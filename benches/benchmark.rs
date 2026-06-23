use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tower_of_hanoi::{tower_of_hanoi, tower_of_hanoi_iterative};

fn benchmark_recursive(c: &mut Criterion) {
    c.bench_function("recursive_10", |b| {
        b.iter(|| {
            let mut moves = Vec::new();
            tower_of_hanoi(black_box(10), 'A', 'C', 'B', &mut moves);
        })
    });
}

fn benchmark_iterative(c: &mut Criterion) {
    c.bench_function("iterative_10", |b| {
        b.iter(|| {
            tower_of_hanoi_iterative(black_box(10), 'A', 'C', 'B');
        })
    });
}

criterion_group!(benches, benchmark_recursive, benchmark_iterative);
criterion_main!(benches);