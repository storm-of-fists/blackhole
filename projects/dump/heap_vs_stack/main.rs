#![feature(test)]

extern crate test;

use test::Bencher;

pub struct Cool {}

impl Cool {
    pub fn hello(&self, yes: i32) -> i32 {
        yes
    }
}

#[bench]
fn bench_hello_on_heap(b: &mut Bencher) {
    let boxed_cool = Box::pin(Cool {});

    b.iter(|| (0..1000).map(|val| boxed_cool.hello(val)));
}
