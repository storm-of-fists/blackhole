use data_trait::DataTrait;
use std::{
    sync::{Arc, Mutex, MutexGuard},
    any::Any,
    collections::BTreeMap,
};
use as_any::AsAny;

/// TODO(AddBTreeMapSettings)
/// TODO(TrackGetsPerUpdate)
/// TODO(AddHashMapCacheForCommonEntities)
pub struct DataSet<T: DataTrait> {
    /// TODO(RealEntityId)
    datas: Arc<Mutex<BTreeMap<usize, T>>>,
}

impl<T> Clone for DataSet<T> where T: DataTrait {
    fn clone(&self) -> Self {
        Self {
            datas: self.datas.clone()
        }
    }
}

impl<T> DataSet<T> where T: DataTrait {
    pub fn new() -> Self {
        Self {
            datas: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    /// TODO(MakeFallible)
    /// TODO(MakeSingularlyMutable?)
    pub fn get(&self) -> MutexGuard<BTreeMap<usize, T>> {
        self.datas.lock().unwrap()
    }
}

unsafe impl<T> Send for DataSet<T> where T: DataTrait {}
unsafe impl<T> Sync for DataSet<T> where T: DataTrait {}

impl<T> AsAny for DataSet<T> where T: DataTrait {
    fn as_any(&self) -> &dyn Any {
        self
    }
}