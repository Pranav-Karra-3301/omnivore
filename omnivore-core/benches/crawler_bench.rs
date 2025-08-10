use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use omnivore_core::crawler::frontier::Frontier;
use url::Url;

fn frontier_benchmark(c: &mut Criterion) {
    c.bench_function("frontier_add_1000_urls", |b| {
        b.iter(|| {
            let mut frontier = Frontier::new();
            for i in 0..1000 {
                let url = Url::parse(&format!("https://example.com/page{i}")).unwrap();
                frontier.add(black_box(url), black_box(0)).unwrap();
            }
        });
    });

    c.bench_function("frontier_get_next_1000", |b| {
        let mut frontier = Frontier::new();
        for i in 0..1000 {
            let url = Url::parse(&format!("https://example.com/page{i}")).unwrap();
            frontier.add(url, i % 10).unwrap();
        }

        b.iter(|| {
            let mut f = frontier.clone();
            for _ in 0..1000 {
                black_box(f.get_next());
            }
        });
    });
}

criterion_group!(benches, frontier_benchmark);
criterion_main!(benches);
