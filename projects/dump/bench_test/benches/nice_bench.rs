use criterion::{black_box, criterion_group, criterion_main, Criterion};

struct Cool {
    pub nice: i32,
}

impl Cool {
    pub fn ok(&self, yes: i32) -> i32 {
        self.nice + yes
    }
}

fn heaped(c: &mut Criterion) {
    let yeah = Box::pin(Cool { nice: 35 });
    c.bench_function("heaped", |b| b.iter(|| yeah.ok(black_box(20))));
}

fn stacked(c: &mut Criterion) {
    let yeah = Cool { nice: 35 };
    c.bench_function("stacked", |b| b.iter(|| yeah.ok(black_box(20))));
}

criterion_group!(benches, heaped, stacked);
criterion_main!(benches);
