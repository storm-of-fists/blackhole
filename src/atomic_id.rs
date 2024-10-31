use std::{ops, sync, fmt};
use uuid;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct AtomicUuid {
    uuid: sync::Arc<uuid::Uuid>,
}

unsafe impl Send for AtomicUuid {}

impl AtomicUuid {
    pub fn new() -> Self {
        AtomicUuid {
            uuid: sync::Arc::new(uuid::Uuid::new_v4())
        }
    }
    
    pub fn from(uuid: uuid::Uuid) -> Self {
        AtomicUuid {
            uuid: sync::Arc::new(uuid),
        }
    }
}

impl ops::Deref for AtomicUuid {
    type Target = uuid::Uuid;
    fn deref(&self) -> &Self::Target {
        &self.uuid
    }
}

impl fmt::Display for AtomicUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
                    id: _GENERATOR.fetch_add(1, atomic::Ordering::SeqCst)
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

fn main() {
    
    let handle1 = std::thread::spawn(|| {
        let id = Id::new();
        println!("thread1! {id}");
    });
    let handle2 = std::thread::spawn(|| {
        let id = Id::new();
        println!("thread2! {id}");
    });
    let handle3 = std::thread::spawn(|| {
        let id = Id::new(); 
        println!("thread3! {id}");
    });
    
    handle3.join();
    handle2.join();
    handle1.join();
}
