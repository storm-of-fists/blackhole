// use std::sync::mpmc::Sender;

// use crate::*;

// #[derive(Debug)]
// pub struct ControlMessages {
//     control_messages: HashMap<String, Sender<NucleusControlMessage>>,
// }

// pub struct NucleusControlMessage {
//     target_updater: TypeId,
//     message: ControlMessageEnum,
// }

// pub enum ControlMessageEnum {
//     ToggleUpdater(ToggleUpdater),
//     SortUpdater(SortUpdater),
// }

// pub struct ToggleUpdater {
//     toggle: bool,
// }

// pub struct SortUpdater {
//     order: u32,
// }

// impl State for ControlMessages {}

// pub struct EntityManager {
//     // entity_id_counter: AtomicU32,
// }

// pub struct TimingManager {}