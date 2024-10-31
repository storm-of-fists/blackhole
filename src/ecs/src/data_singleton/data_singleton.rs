use data_trait::DataTrait;
use std::{
    sync::{Arc, Mutex, MutexGuard},
    any::Any,
};
use as_any::AsAny;

pub struct DataSingleton<T> where T: DataTrait {
    /// TODO(TrackGetsPerUpdate)
    /// +
    data: Arc<Mutex<T>>,
}

impl<T> DataSingleton<T> where T: DataTrait {
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(Mutex::new(data)),
        }
    }

    /// TODO(MakeFallible)
    pub fn get(&self) -> MutexGuard<T> {
        self.data.lock().unwrap()
    }
}

impl<T> Clone for DataSingleton<T> where T: DataTrait {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone()
        }
    }
}

unsafe impl<T> Send for DataSingleton<T> where T: DataTrait {}
unsafe impl<T> Sync for DataSingleton<T> where T: DataTrait {}

impl <T> AsAny for DataSingleton<T> where T: DataTrait {
    fn as_any(&self) -> &dyn Any {
        self
    }
}