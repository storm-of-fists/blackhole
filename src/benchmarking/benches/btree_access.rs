use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, Mutex},
    time::Duration,
    ops::Deref,
    fmt::{Display, Formatter, Result}
};
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct AtomicUuid {
    uuid: Arc<Uuid>,
}

unsafe impl Send for AtomicUuid {}

impl AtomicUuid {
    pub fn new() -> Self {
        AtomicUuid {
            uuid: Arc::new(Uuid::new_v4())
        }
    }

    pub fn from(uuid: Uuid) -> Self {
        AtomicUuid {
            uuid: Arc::new(uuid),
        }
    }
}

impl Deref for AtomicUuid {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.uuid
    }
}

impl Display for AtomicUuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.uuid)
    }
}

macro_rules! atomic_id_generator {
    ($id_name:ident) => {
        use std::{fmt as mac_fmt, ops as mac_ops, sync::atomic as mac_atomic};

        pub static _GENERATOR: mac_atomic::AtomicUsize = mac_atomic::AtomicUsize::new(0);

        #[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
        pub struct $id_name {
            id: usize,
        }

        impl $id_name {
            pub fn new() -> Self {
                Self {
                    id: _GENERATOR.fetch_add(1, mac_atomic::Ordering::SeqCst)
                }
            }
        }

        impl mac_ops::Deref for $id_name {
            type Target = usize;
            fn deref(&self) -> &Self::Target {
                &self.id
            }
        }

        impl mac_fmt::Display for $id_name {
            fn fmt(&self, f: &mut mac_fmt::Formatter<'_>) -> mac_fmt::Result {
                write!(f, "{}", &self.id)
            }
        }
    }
}

atomic_id_generator!(Id);

#[derive(Debug)]
struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
}

impl Position {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            ax: 0.0,
            ay: 0.0,
            az: 0.0,
        }
    }
}

fn map_accesses(c: &mut Criterion) {
    const ITEM_COUNT: u64 = 10000;

    let mut hash_map = HashMap::new();
    let mut btree_map = BTreeMap::new();

    for i in 0..ITEM_COUNT {
        hash_map.insert(i, Position::new());
        btree_map.insert(i, Position::new());
    }

    c.bench_function("random u64 between 0 and 10000", |b| {
        b.iter(|| rand::thread_rng().gen_range(0..ITEM_COUNT))
    });

    c.bench_function("btree map get", |b| {
        b.iter(|| btree_map.get(black_box(&rand::thread_rng().gen_range(0..ITEM_COUNT))))
    });

    c.bench_function("btree map values mut update", |b| {
        b.iter(|| {
            for position in btree_map.values_mut() {
                position.x += 1.0;
            }
        });
    });

    c.bench_function("hash map get", |b| {
        b.iter(|| hash_map.get(black_box(&rand::thread_rng().gen_range(0..ITEM_COUNT))))
    });

    c.bench_function("hash map values mut update", |b| {
        b.iter(|| {
            for position in hash_map.values_mut() {
                position.x += 1.0;
            }
        });
    });
}

fn mutex_unlocks(c: &mut Criterion) {
    let arc_mut_position = Arc::new(Mutex::new(Position::new()));
    let arc_clone = arc_mut_position.clone();

    std::thread::spawn(move || {
        arc_clone;
        loop {
            std::thread::sleep(Duration::from_millis(1));
        }
    });

    c.bench_function("optimistic mut lock", |b| {
        b.iter(|| arc_mut_position.lock());
    });
}

fn atomid_ids(c: &mut Criterion) {
    c.bench_function("create new atomic uuid", |b| {
        b.iter(|| AtomicUuid::new());
    });

    c.bench_function("create new atomic id", |b| {
        b.iter(|| Id::new());
    });

    c.bench_function("create new atomic id in many threads", |b| {
        let mut handles = Vec::new();
        for _ in 0..50 {
            handles.push(std::thread::spawn(|| {
                loop {
                    Id::new();
                    std::thread::sleep(Duration::from_millis(10));
                }
            }));
        }

        b.iter(|| Id::new());
    });
}

criterion_group!(map_benches, map_accesses);
criterion_group!(mutex_benches, mutex_unlocks);
criterion_group!(atomid_id_benches, atomid_ids);

criterion_main!(atomid_id_benches);
