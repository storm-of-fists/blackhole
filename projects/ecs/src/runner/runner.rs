use nucleus::Nucleus;
use function::UpdaterTrait;
use data_trait::DataTrait;
use data_singleton::DataSingleton;
use data_set::DataSet;
use std::{
    time::Duration,
};

pub struct Runner {
    nucleus: Nucleus,
    /// TODO(UpdaterSchedulingAndDependencies)
    functions: Vec<Box<dyn UpdaterTrait>>,
}

impl Runner {
    pub fn new(name: impl Into<String>, nucleus: Nucleus) -> Self {
        Self {
            nucleus,
            functions: Vec::new(),
        }
    }

    pub fn update(&self) {
        for function in self.functions.iter() {
            function.update();
        }
    }

    pub fn run(&self) {
        loop {
            self.update();
        }
    }

    pub fn register_updater<T: UpdaterTrait>(&mut self) {
        T::register(self.nucleus.clone());
    }

    pub fn add_updater<T: UpdaterTrait>(&mut self) {

        self.functions.push(Box::new(T::new(self.nucleus.clone())));
    }

    pub fn add_data_singleton<T: DataTrait>(&self, data: T) -> DataSingleton<T> {
        self.nucleus.add_data_singleton::<T>(data)
    }

    /// TODO(MakeFallible)
    pub fn get_data_singleton<T: DataTrait>(&self) -> DataSingleton<T> {
        self.nucleus.get_data_singleton::<T>()
    }

    /// TODO(MakeFallible)
    pub fn add_data_set<T: DataTrait>(&self) -> DataSet<T> {
        self.nucleus.add_data_set::<T>()
    }

    /// TODO(MakeFallible)
    pub fn get_data_set<T: DataTrait>(&self) -> DataSet<T> {
        self.nucleus.get_data_set::<T>()
    }
}