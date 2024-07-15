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

// https://www.reddit.com/r/rust/comments/kkap4e/how_to_cast_a_boxdyn_mytrait_to_an_actual_struct/
// https://www.reddit.com/r/learnrust/comments/uj68qn/how_do_i_use_downcast_ref_correctly/

// 0. all data should be a component, components are updated in systems, systems are stored in trees, trees can contain handles to other trees
// 1. do not compose components, simple add a new component for that specific state
// 2. all components are stored in btrees or as singletons
// 3. systems should declare all their existing and future component set types at the start, even if empty for a while
// 4. only manipulate the tree when changing the structure of the program, not anything to do with data
// 5. tree manipulations should be fallible since those fail at the start

use anyhow::{Result, anyhow}; // 1.0.86

pub trait ComponentTrait: Debug + Clone + Sized + 'static {}

#[derive(Clone)]
pub struct ComponentSet<T: ComponentTrait> {
    components: Arc<Mutex<BTreeMap<usize, T>>>,
}

impl<T> ComponentSet<T> where T: ComponentTrait {
    pub fn new() -> Self {
        Self {
            components: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }
    
    pub fn get(&self) -> MutexGuard<BTreeMap<usize, T>> {
        self.components.lock().unwrap()
    }
}

unsafe impl<T> Send for ComponentSet<T> where T: ComponentTrait {}
unsafe impl<T> Sync for ComponentSet<T> where T: ComponentTrait {}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T> AsAny for ComponentSet<T> where T: ComponentTrait {
    fn as_any(&self) -> &dyn Any {
        self
    }
}


#[derive(Clone)]
pub struct SingletonComponent<T> where T: ComponentTrait {
    component: Arc<Mutex<T>>,
}

impl<T> SingletonComponent<T> where T: ComponentTrait {
    pub fn new(data: T) -> Self {
        Self {
            component: Arc::new(Mutex::new(data)),
        }
    }
    
    pub fn get(&self) -> MutexGuard<T> {
        self.component.lock().unwrap()
    }
}

unsafe impl<T> Send for SingletonComponent<T> where T: ComponentTrait {}
unsafe impl<T> Sync for SingletonComponent<T> where T: ComponentTrait {}

impl <T> AsAny for SingletonComponent<T> where T: ComponentTrait {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait SystemTrait {
    fn new(tree: &mut Tree) -> Self where Self: Sized;
    fn update(&self);
}

pub struct RootData {
    name: String,
}

pub struct TreeData {
    name: String,
}

pub struct Tree {
    root: Arc<Mutex<RootData>>,
    parent_tree: Option<Arc<Mutex<TreeData>>>,
    data: Arc<Mutex<TreeData>>,
    
    component_sets: HashMap<TypeId, Box<dyn AsAny>>,
    singleton_components: HashMap<TypeId, Box<dyn AsAny>>,
    
    systems: Vec<Box<dyn SystemTrait>>,
    
    sub_tree_thread_handles: HashMap<&'static str, JoinHandle<()>>,
}

impl Tree {
    pub fn new(name: impl Into<String>, root: Arc<Mutex<RootData>>, parent_tree: Option<Arc<Mutex<TreeData>>>) -> Self {
        Self {
            root,
            data: Arc::new(Mutex::new(TreeData {
                name: name.into()
            })),
            parent_tree,
            component_sets: HashMap::new(),
            singleton_components: HashMap::new(),
            systems: Vec::new(),
            sub_tree_thread_handles: HashMap::new()
        }
    }
    
    pub fn update(&self) {
        for system in self.systems.iter() {
            system.update();
        }
    }
    
    pub fn run(&self) {
        loop {
            self.update();
            println!("run!");
            std::thread::sleep(Duration::from_secs(1));
        }
    }
    
    pub fn add_system<T: SystemTrait + 'static>(&mut self) {
        let new_system = Box::new(T::new(self));
        self.systems.push(new_system);
    }
    
    pub fn add_singleton_component<T: ComponentTrait + 'static>(&mut self, data: T) -> SingletonComponent<T> {
        let singleton_component = SingletonComponent::new(data);
        self.singleton_components.insert(TypeId::of::<T>(), Box::new(singleton_component.clone()));
        
        singleton_component
    }
    
    pub fn get_singleton_component<T: ComponentTrait + 'static>(&mut self) -> SingletonComponent<T> {
        let box_ref = self.singleton_components.get(&TypeId::of::<T>()).unwrap();
        
        (*box_ref.as_any().downcast_ref::<SingletonComponent<T>>().unwrap()).clone()
    }
    
    pub fn get_component_set<T: ComponentTrait>(&mut self) -> ComponentSet<T> {
        let component_type_id = TypeId::of::<T>();
        
        if !self.component_sets.contains_key(&component_type_id) {
            self.component_sets.insert(component_type_id.clone(), Box::new(ComponentSet::<T>::new()));    
        }
        
        let box_ref = self.component_sets.get(&component_type_id).unwrap();
        
        (*box_ref.as_any().downcast_ref::<ComponentSet<T>>().unwrap()).clone()
    }
}

#[derive(Clone, Debug)]
pub struct MoverComponent {
    x: f32,
    y: f32,
    z: f32,
    vx: f32,
    vy: f32,
    vz: f32,
    ax: f32,
    ay: f32,
    az: f32,
}
impl ComponentTrait for MoverComponent {}

#[derive(Clone, Debug)]
pub struct MovementSystemData {
    update_count: u32,
}

impl ComponentTrait for MovementSystemData {}

pub struct MovementSystem {
    data: SingletonComponent<MovementSystemData>,
    movers: ComponentSet<MoverComponent>,
}

impl SystemTrait for MovementSystem {
    fn new(tree: &mut Tree) -> Self where Self: Sized {
        Self {
            data: tree.add_singleton_component(MovementSystemData {
                update_count: 0,
            }),
            movers: tree.get_component_set::<MoverComponent>(),
        }
    }
    
    fn update(&self) {
        let mut data = self.data.get();
        let mut movers = self.movers.get();
        
        println!("{:?}", data);
        println!("{:?}", movers);
        
        movers.insert(data.update_count as usize, MoverComponent {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            ax: 0.0,
            ay: 0.0,
            az: 0.0
        });
        
        data.update_count += 1;
    }
}

fn main() {
    let root_data = Arc::new(Mutex::new(RootData {
        name: "root".to_string()
    }));
    
    let mut primary = Tree::new("root", root_data, None);
    
    primary.add_system::<MovementSystem>();
    
    primary.run();
}

// pub trait ComponentTraitPrivate {
//     fn entity_id(&self) -> usize;
// }

// pub trait ComponentTrait: Any + Sized {
//     fn as_any(&self) -> &dyn Any;
// }

// pub struct Component<T> where T: ComponentTrait {
//     entity_id: usize,
//     inner: T,
// }

// impl<T> Deref for Component<T> where T: ComponentTrait {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

// impl<T> DerefMut for Component<T> where T: ComponentTrait {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.inner
//     }
// }

// impl<T> ComponentTraitPrivate for Component<T> where T: ComponentTrait {
//     fn entity_id(&self) -> usize {
//         self.entity_id
//     }
// }

// impl<T> Any for Component<T> where T: ComponentTrait {
//     fn type_id(&self) -> TypeId {
//         self.inner.type_id()
//     }
// }


// pub trait ComponentSetTrait: Any {
//     fn as_any(&self) -> &dyn Any;
// }

// impl<T> ComponentSetTrait for ComponentSet<T> where T: ComponentSetTrait {
//     fn as_any(&self) -> &dyn Any {
//         &self
//     }
// }


// // pub trait ComponentSetTraitPrivate {
// //     fn as_any(&self) -> &dyn Any;
// // }

// pub struct ComponentSet<T> where T: ComponentTrait {
//     set: Mutex<BTreeMap<usize, Component<T>>>,
// }

// // impl<T> ComponentSet 

// pub struct Context {
//     name: String,
//     component_sets: HashMap<TypeId, Arc<dyn ComponentSetTrait>>,   
// }

// impl Context {
//     fn register_component_set<T: ComponentTrait>(&mut self, component_set: ComponentSet<T>) {
//         self.component_sets.insert(TypeId::of::<T>(), Arc::new(component_set));
//     }
    
//     fn get_component_set<T: ComponentSetTrait>(&self) -> Result<Arc<T>> {
//         let type_id = TypeId::of::<T>();
        
//         let Some(component_set) = self.component_sets.get(type_id) else {
//             return Err(anyhow!("shite!"));
//         };

//         match component_set.downcast_ref::<T>() {
//             Some(i) => Ok(i),
//             None => Err(anyhow!("shite!")),
//         }
//     }
// }


// pub struct Context2 {
//     name: String,
    
// }

// pub trait ComponentSetTrait {
//     fn get_components(&self) -> &BTreeMap<usize, T>;
// }

// pub struct ComponentSet<T: ComponentTraitPrivate> {
//     set: Mutex<BTreeMap<usize, T>>,
// }

// pub trait System {
    
//     fn update(&self);
// }










// pub struct Context {
//     name: String,
//     rc_components: HashMap<TypeId, BTreeMap<usize, RcComponent>>,
//     // arc_components: HashMap<TypeId, BTreeMap<usize, ArcComponent>>,
//     child_contexts: HashMap<String, JoinHandle<()>>,
//     systems: Vec<Box<dyn Fn(&Self)->()>>,
//     // read_only_arc_components: HashMap<&'static str, BTreeMap<usize, dyn ReadOnlyArcComponent>>,
//     // mutable_rc_components: HashMap<&'static str, BTreeMap<usize, dyn MutableRcComponent>>,
//     // mutable_arc_components: HashMap<&'static str, BTreeMap<usize, dyn MutableArcComponent>>,
// }

// impl Context {
//     pub fn new(name: impl Into<String>) -> Self {
//         Self {
//             name: name.into(),
//             child_contexts: HashMap::new(),
//             rc_components: HashMap::new(),
//             systems: Vec::new(),
//         }
//     }
    
//     pub fn add_child_context<F>(&mut self, name: impl Into<String>, func: F)
//         where F: FnOnce() + Send + 'static
//     {
//         self.child_contexts.insert(name.into(), spawn(func));
//     }
    
//     pub fn get_rc_components<T: ComponentTrait>(&self) -> Option<&BTreeMap<usize, RcComponent>> {
//         self.rc_components.get(&TypeId::of::<T>())
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

// pub trait ComponentTrait: Any {
//     fn new(ctx: &Context, entity_id: usize) -> Self where Self: Sized;
//     fn as_any(&self) -> &dyn Any;
// }

// #[derive(Debug)]
// pub struct Position {
//     x: f32,
//     y: f32,
//     z: f32,
// }

// impl ComponentTrait for Position {
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
//     let Some(component_tree) = ctx.get_rc_components::<Position>() else {
//         return;
//     };
    
//     for position_component in component_tree.values() {
//         let Ok(position) = position_component.get::<Position>() else {
//             return;
//         };
        
//         println!("{:?}", position);
//     }
// }

// pub struct RcComponent {
//     inner: Rc<RefCell<dyn ComponentTrait>>,
// }

// impl RcComponent {
//     pub fn get<T: ComponentTrait>(&self) -> Result<&T> {
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

// pub struct ExternalComponent {
//     inner: Arc<Mutex<dyn ComponentTrait>>,
// }

// impl Component {
    
// }

// /// Runs in the main thread.
// pub struct InputCommands {
//     input_buffer: Vec<Commands>
// }

// pub trait ComponentTrait {
//     fn entity_id(&self) -> usize;
//     fn type_name(&self) -> &'static str;
// }

// pub trait ReadOnlyComponent: ConstComponent {
//     /// Update this individual component. By default this just does
//     /// nothing.
//     
// }

// pub trait MutableComponent: ReadOnlyComponent {
//     
// }

// pub struct Component {
//     inner: Rc<RefCell<dyn ComponentTrait>>,
// }

// impl Component {
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



// pub struct Component {
//     entity_id: usize,
//     data: Rc<RefCell<dyn InnerComponentTrait>>,
// }

// impl<T> Component<T> where T: Sized {
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
//     /// all along a component tree.
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
//     components: HashMap<&'static str, BTreeMap<u64, dyn ComponentTrait>>,
//     systems: Vec<Box<dyn Fn(&Simulation)>>
// }

// impl Simulation {
//     pub fn register_component<T: ComponentTrait>(component: T)
// }

// pub struct Position {
//     x: f32,
//     y: f32,
//     z: f32,
// }

// impl ComponentTrait for Component<Position> {
//     fn entity_id(&self) -> usize {
//         self.entity_id
//     }
    
//     fn type_name(&self) -> &'static str {
//         "position"
//     }
// }

// // pub struct ThreadsafeComponent<T> {
// //     data: Arc<Mutex<T>>,
// // }

// // impl<T> ThreadsafeComponent<T> where T: Sized {
    
// // }

// fn main() {
    
// }
