// benches/my_bench.rs
use criterion::{Criterion, black_box, criterion_group, criterion_main};

/// Sums all integers from 0 to n.
fn sum_to(n: u64) -> u64 {
    (0..n).sum()
}

fn bench_sum(c: &mut Criterion) {
    c.bench_function("sum_to_1000", |b| {
        // black_box prevents the compiler from optimizing the input away.
        // The compiler cannot assume the value of black_box(1000).
        b.iter(|| sum_to(black_box(1000)))
    });
}

// Generates the main function and registers the benchmark group.
criterion_group!(benches, bench_sum);
criterion_main!(benches);
