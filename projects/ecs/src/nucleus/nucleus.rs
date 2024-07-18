use as_any::AsAny;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    any::TypeId,
};

use data_trait::DataTrait;
use data_singleton::DataSingleton;
use data_set::DataSet;

struct NucleusData {
    name: String,

    data_set_map: HashMap<TypeId, Box<dyn AsAny>>,

    data_singleton_map: HashMap<TypeId, Box<dyn AsAny>>,

    runner_map: HashMap<String, Box<dyn AsAny>>,
}

pub struct Nucleus {
    data: Arc<Mutex<NucleusData>>,
}

impl Nucleus {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            data: Arc::new(Mutex::new(NucleusData {
                name: name.into(),
                data_set_map: HashMap::new(),
                data_singleton_map: HashMap::new(),
                runner_map: HashMap::new(),
            }))
        }
    }

    /// TODO(MakeFallible)
    pub fn add_data_singleton<T: DataTrait>(&self, data: T) -> DataSingleton<T> {
        let mut nucleus = self.data.lock().unwrap();

        let data_singleton = DataSingleton::new(data);

        nucleus.data_singleton_map.insert(TypeId::of::<T>(), Box::new(data_singleton.clone()));

        data_singleton
    }

    /// TODO(MakeFallible)
    /// TODO(HandleSingleMutability)
    /// Possible ways to do single mutability are keep track of what
    /// data have already been given out mutably. would need a new
    /// data singleton primitive?
    pub fn get_data_singleton<T: DataTrait>(&self) -> DataSingleton<T> {
        let nucleus = self.data.lock().unwrap();

        let box_ref = nucleus.data_singleton_map.get(&TypeId::of::<T>()).unwrap();

        (*box_ref.as_any().downcast_ref::<DataSingleton<T>>().unwrap()).clone()
    }

    /// TODO(MakeFallible)
    pub fn add_data_set<T: DataTrait>(&self) -> DataSet<T> {
        let mut nucleus = self.data.lock().unwrap();

        let data_set = DataSet::<T>::new();
        nucleus.data_set_map.insert(TypeId::of::<T>(), Box::new(data_set.clone()));

        data_set
    }

    /// TODO(MakeFallible)
    /// TODO(HandleSingleMutability)
    pub fn get_data_set<T: DataTrait>(&self) -> DataSet<T> {
        let nucleus = self.data.lock().unwrap();

        let box_ref = nucleus.data_set_map.get(&TypeId::of::<T>()).unwrap();

        (*box_ref.as_any().downcast_ref::<DataSet<T>>().unwrap()).clone()
    }
}

unsafe impl Send for Nucleus {}
unsafe impl Sync for Nucleus {}

impl Clone for Nucleus {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone()
        }
    }
}