/// I keep this file around to remind me of all the attempts (successful or not)
/// I made trying to do this.

use std::{
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
    collections::{HashMap, BTreeMap},
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard},
    cell::{RefMut, Ref, RefCell},
    thread::{spawn, JoinHandle},
    any::{Any, TypeId},
    ops::{Deref, DerefMut},
    fmt::Debug,
};


use anyhow::{Result, anyhow}; // 1.0.86

// pub trait DataTraitPrivate {
//     fn entity_id(&self) -> usize;
// }

// pub trait DataTrait: Any + Sized {
//     fn as_any(&self) -> &dyn Any;
// }

// pub struct Data<T> where T: DataTrait {
//     entity_id: usize,
//     inner: T,
// }

// impl<T> Deref for Data<T> where T: DataTrait {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

// impl<T> DerefMut for Data<T> where T: DataTrait {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.inner
//     }
// }

// impl<T> DataTraitPrivate for Data<T> where T: DataTrait {
//     fn entity_id(&self) -> usize {
//         self.entity_id
//     }
// }

// impl<T> Any for Data<T> where T: DataTrait {
//     fn type_id(&self) -> TypeId {
//         self.inner.type_id()
//     }
// }


// pub trait DataSetTrait: Any {
//     fn as_any(&self) -> &dyn Any;
// }

// impl<T> DataSetTrait for DataSet<T> where T: DataSetTrait {
//     fn as_any(&self) -> &dyn Any {
//         &self
//     }
// }


// // pub trait DataSetTraitPrivate {
// //     fn as_any(&self) -> &dyn Any;
// // }

// pub struct DataSet<T> where T: DataTrait {
//     set: Mutex<BTreeMap<usize, Data<T>>>,
// }

// // impl<T> DataSet

// pub struct Context {
//     name: String,
//     data_sets: HashMap<TypeId, Arc<dyn DataSetTrait>>,
// }

// impl Context {
//     fn register_Data_set<T: DataTrait>(&mut self, Data_set: DataSet<T>) {
//         self.data_sets.insert(TypeId::of::<T>(), Arc::new(Data_set));
//     }

//     fn get_Data_set<T: DataSetTrait>(&self) -> Result<Arc<T>> {
//         let type_id = TypeId::of::<T>();

//         let Some(Data_set) = self.data_sets.get(type_id) else {
//             return Err(anyhow!("shite!"));
//         };

//         match Data_set.downcast_ref::<T>() {
//             Some(i) => Ok(i),
//             None => Err(anyhow!("shite!")),
//         }
//     }
// }


// pub struct Context2 {
//     name: String,

// }

// pub trait DataSetTrait {
//     fn get_Datas(&self) -> &BTreeMap<usize, T>;
// }

// pub struct DataSet<T: DataTraitPrivate> {
//     set: Mutex<BTreeMap<usize, T>>,
// }

// pub trait System {

//     fn update(&self);
// }










// pub struct Context {
//     name: String,
//     rc_Datas: HashMap<TypeId, BTreeMap<usize, RcData>>,
//     // arc_Datas: HashMap<TypeId, BTreeMap<usize, ArcData>>,
//     child_contexts: HashMap<String, JoinHandle<()>>,
//     systems: Vec<Box<dyn Fn(&Self)->()>>,
//     // read_only_arc_Datas: HashMap<&'static str, BTreeMap<usize, dyn ReadOnlyArcData>>,
//     // mutable_rc_Datas: HashMap<&'static str, BTreeMap<usize, dyn MutableRcData>>,
//     // mutable_arc_Datas: HashMap<&'static str, BTreeMap<usize, dyn MutableArcData>>,
// }

// impl Context {
//     pub fn new(name: impl Into<String>) -> Self {
//         Self {
//             name: name.into(),
//             child_contexts: HashMap::new(),
//             rc_Datas: HashMap::new(),
//             systems: Vec::new(),
//         }
//     }

//     pub fn add_child_context<F>(&mut self, name: impl Into<String>, func: F)
//         where F: FnOnce() + Send + 'static
//     {
//         self.child_contexts.insert(name.into(), spawn(func));
//     }

//     pub fn get_rc_Datas<T: DataTrait>(&self) -> Option<&BTreeMap<usize, RcData>> {
//         self.rc_Datas.get(&TypeId::of::<T>())
//     }

//     pub fn add_system<F>(&mut self, func: F)
//         where F: Fn(&Self) + 'static
//     {
//         self.systems.push(Box::new(func));
//     }

//     pub fn run(&self) {
//         loop {
//             for system in self.systems.iter() {
//                 system(&self);
//             }
//         }
//     }
// }

// fn main() {
//     let mut context = Context::new("main");

//     context.add_child_context("input_context", || {
//         Context::new("input").run();
//     });

//     context.add_system(update_position);

//     context.run();
// }

// // Runs in a user input thread.
// // pub struct KeyStrokeInput {
// //     input_buffer: Vec<KeyStroke>
// // }

// pub trait DataTrait: Any {
//     fn new(ctx: &Context, entity_id: usize) -> Self where Self: Sized;
//     fn as_any(&self) -> &dyn Any;
// }

// #[derive(Debug)]
// pub struct Position {
//     x: f32,
//     y: f32,
//     z: f32,
// }

// impl DataTrait for Position {
//     fn new(_ctx: &Context, _entity_id: usize) -> Self {
//         Self {
//             x: 0.0,
//             y: 0.0,
//             z: 0.0,
//         }
//     }

//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

// pub fn update_position(ctx: &Context) {
//     let Some(Data_tree) = ctx.get_rc_Datas::<Position>() else {
//         return;
//     };

//     for position_Data in Data_tree.values() {
//         let Ok(position) = position_Data.get::<Position>() else {
//             return;
//         };

//         println!("{:?}", position);
//     }
// }

// pub struct RcData {
//     inner: Rc<RefCell<dyn DataTrait>>,
// }

// impl RcData {
//     pub fn get<T: DataTrait>(&self) -> Result<&T> {
//         Ok(
//             (*(
//                 self
//                 .inner
//                 .try_borrow()
//                 .map_err(|_| Err(anyhow!("shoot")))?
//             ))
//             .as_any()
//             .downcast_ref::<T>().ok_or(
//                 Err(anyhow!("shite!"))
//             )?
//         )

//     }
// }

// pub struct ExternalData {
//     inner: Arc<Mutex<dyn DataTrait>>,
// }

// impl Data {

// }

// /// Runs in the main thread.
// pub struct InputCommands {
//     input_buffer: Vec<Commands>
// }

// pub trait DataTrait {
//     fn entity_id(&self) -> usize;
//     fn type_name(&self) -> &'static str;
// }

// pub trait ReadOnlyData: ConstData {
//     /// Update this individual Data. By default this just does
//     /// nothing.
//
// }

// pub trait MutableData: ReadOnlyData {
//
// }

// pub struct Data {
//     inner: Rc<RefCell<dyn DataTrait>>,
// }

// impl Data {
//     fn get(&self) -> Result<Ref<Self>> {
//
//     }

//     fn get_mut(&mut self) -> Result<RefMut<Self>> {
//         match self.inner.try_borrow_mut() {
//             Ok(inner) => Ok(inner),
//             Err(_) => Err(anyhow!("shoot!"))
//         }
//     }

//     fn try_clone(&self) -> Result<Self> {
//         if self.get().is_ok() {
//             Ok(Self {
//                 inner: self.inner.clone()
//             })
//         } else {
//             Err(anyhow!("cant clone"))
//         }
//     }

//     fn update(&self, context: &Context) {}
// }



// pub struct Data {
//     entity_id: usize,
//     data: Rc<RefCell<dyn InnerDataTrait>>,
// }

// impl<T> Data<T> where T: Sized {
//     pub fn new(entity_id: usize, data: T) -> Self {
//         Self {
//             entity_id,
//             data: Rc::new(RefCell::new(data))
//         }
//     }

//     pub fn from_rc(entity_id: usize, data: Rc<RefCell<T>>) -> Self {
//         Self {
//             entity_id,
//             data,
//         }
//     }

//     pub fn get(&self) -> Result<Ref<T>> {
//         if let Ok(data) = self.data.try_borrow() {
//             Ok(data)
//         } else {
//             Err(anyhow!("Unable to borrow!"))
//         }
//     }

//     /// This may seem like it should be mutable, but we are going through
//     /// the refcell for it. We keep this a &self reference so we can mutate
//     /// all along a Data tree.
//     pub fn get_mut(&self) -> Result<RefMut<T>> {
//         if let Ok(data) = self.data.try_borrow_mut() {
//             Ok(data)
//         } else {
//             Err(anyhow!("Unable to borrow mut!"))
//         }
//     }

//     /// Implement try_clone to avoid panicking.
//     pub fn try_clone(&self) -> Result<Self> {
//         if self.get().is_ok() {
//             return Ok(Self {
//                 entity_id: self.entity_id,
//                 data: self.data.clone(),
//             })
//         } else {
//             return Err(anyhow!("Unable to clone!"))
//         }
//     }

//     pub fn instance_count(&self) -> usize {
//         Rc::strong_count(&self.data)
//     }

//     pub fn entity_id(&self) -> usize {
//         self.entity_id
//     }
// }

// pub struct Simulation {
//     Datas: HashMap<&'static str, BTreeMap<u64, dyn DataTrait>>,
//     systems: Vec<Box<dyn Fn(&Simulation)>>
// }

// impl Simulation {
//     pub fn register_Data<T: DataTrait>(Data: T)
// }

// pub struct Position {
//     x: f32,
//     y: f32,
//     z: f32,
// }

// impl DataTrait for Data<Position> {
//     fn entity_id(&self) -> usize {
//         self.entity_id
//     }

//     fn type_name(&self) -> &'static str {
//         "position"
//     }
// }

// // pub struct ThreadsafeData<T> {
// //     data: Arc<Mutex<T>>,
// // }

// // impl<T> ThreadsafeData<T> where T: Sized {

// // }

// fn main() {

// }
