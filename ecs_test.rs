use std::{
    time::{Duration, Instant, SystemTime, UNIX_EPOCH}, 
    collections::{HashMap, BTreeMap},
    rc::Rc,
    sync::{Arc, Mutex},
    cell::{RefMut, Ref, RefCell},
    thread::{spawn, JoinHandle},
    any::{Any, TypeId},
    ops::{Deref, DerefMut},
};

// https://www.reddit.com/r/rust/comments/kkap4e/how_to_cast_a_boxdyn_mytrait_to_an_actual_struct/

use anyhow::{Result, anyhow}; // 1.0.86

#[derive(Clone)]
pub struct MoverComponent {}
impl Component for MoverComponent {}

#[derive(Clone)]
pub struct PositionComponent {}
impl Component for PositionComponent {}

#[derive(Clone)]
pub struct VelocityComponent {}
impl Component for VelocityComponent {}

#[derive(Clone)]
pub struct AccelerationComponent {}
impl Component for AccelerationComponent {}

pub trait Component: Clone + Sized + 'static {}

#[derive(Clone)]
pub struct ComponentSet<T: Component> {
    components: Arc<Mutex<BTreeMap<usize, T>>>,
}

impl<T> ComponentSet<T> where T: Component {
    pub fn new() -> Self {
        Self {
            components: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }
}

unsafe impl<T> Send for ComponentSet<T> where T: Component {}
unsafe impl<T> Sync for ComponentSet<T> where T: Component {}

pub trait ComponentSetAsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T> ComponentSetAsAny for ComponentSet<T> where T: Component + Sized {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct MovementSystem {
    movers: ComponentSet<MoverComponent>,
    positions: ComponentSet<PositionComponent>,
    velocities: ComponentSet<VelocityComponent>,
    accelerations: ComponentSet<AccelerationComponent>,
}

// pub struct System<T> where T: SystemTrait {
//     inner: T
// }

// pub trait PrivateSystemTrait {
//     fn update(&self);
// }

// impl<T> PrivateSystemTrait for System<T> where T: SystemTrait {
//     fn update(&self) {
//         self.inner.update();
//     }
// }

pub trait SystemTrait {
    fn new(tree: &mut Tree) -> Self where Self: Sized;
    fn update(&self);
}

impl SystemTrait for MovementSystem {
    fn new(tree: &mut Tree) -> Self where Self: Sized {
        Self {
            movers: tree.get_component_set::<MoverComponent>(),
            positions: tree.get_component_set::<PositionComponent>(),
            velocities: tree.get_component_set::<VelocityComponent>(),
            accelerations: tree.get_component_set::<AccelerationComponent>(),
        }
    }
    
    fn update(&self) {
    }
}

// impl SystemTrait for MovementSystem {
//     fn new(tree: &mut Tree) -> Self where Self: Sized {
//         MovementSystem {
//             movers: tree.get_component_set::<MoverComponent>(),
//             positions: tree.get_component_set::<PositionComponent>(),
//             velocities: tree.get_component_set::<VelocityComponent>(),
//             accelerations: tree.get_component_set::<AccelerationComponent>(),
//         }
//     }

//     fn update(&self) {}
// }

pub struct RootData {
    name: String,
}

pub struct Root {
    data: Arc<Mutex<RootData>>,
    tree: Tree,
}

impl Root {
    pub fn new(name: impl Into<String>) -> Self {
        let root_data = Arc::new(Mutex::new(RootData {
            name: name.into(),
        }));
        
        Self {
            data: root_data.clone(),
            tree: Tree::new("root", root_data),
        }
    }
    
    pub fn get_component_set<T: Component>(&mut self) -> ComponentSet<T> {
        self.tree.get_component_set::<T>()
    }
    
    pub fn add_system<T: SystemTrait + 'static>(&mut self, system: T) {
        self.tree.add_system(system);
    }
    
    pub fn add_sub_tree(&mut self, _tree: Tree) {
        // self.tree.
    }
    
    pub fn tree_mut(&mut self) -> &mut Tree {
        &mut self.tree
    }
    
    pub fn run(&self) {
        loop {
            self.tree.update();
            std::thread::sleep(Duration::from_secs(1));
            println!("ran loop");
        }
    }
}

pub struct TreeData {
    name: String,
}

pub struct Tree {
    root: Arc<Mutex<RootData>>,
    
    data: Arc<Mutex<TreeData>>,
    
    component_sets: HashMap<TypeId, Box<dyn ComponentSetAsAny>>,
    
    systems: Vec<Box<dyn SystemTrait>>,
    
    sub_tree_thread_handles: HashMap<&'static str, JoinHandle<()>>,
}

impl Tree {
    pub fn new(name: impl Into<String>, root: Arc<Mutex<RootData>>) -> Self {
        Self {
            root,
            data: Arc::new(Mutex::new(TreeData {
                name: name.into()
            })),
            component_sets: HashMap::new(),
            systems: Vec::new(),
            sub_tree_thread_handles: HashMap::new()
        }
    }
    
    pub fn update(&self) {
        for system in self.systems.iter() {
            system.update();
        }
    }
    
    pub fn add_system<T: SystemTrait + 'static>(&mut self, system: T) {
        self.systems.push(Box::new(system));
    }
    
    pub fn get_component_set<T: Component>(&mut self) -> ComponentSet<T> {
        let component_type_id = TypeId::of::<T>();
        
        if !self.component_sets.contains_key(&component_type_id) {
            self.component_sets.insert(component_type_id.clone(), Box::new(ComponentSet::<T>::new()));    
        }
        
        let box_ref = self.component_sets.get(&component_type_id).unwrap();
        
        (*box_ref.as_any().downcast_ref::<ComponentSet<T>>().unwrap()).clone()
    }
}

fn main() {
    let mut root = Root::new("root");
    
    let movement_system = MovementSystem::new(root.tree_mut());
    
    root.add_system(movement_system);
    
    root.run();
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
