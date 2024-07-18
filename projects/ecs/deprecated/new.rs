struct Nucleus {
    
}

pub trait ComponentTrait: Sized + 'static {
    fn new(nucleus: Nucleus) -> Self;
    fn update(&self);
}

pub struct MyComponent {
    another_component: Component<Movement>,
}