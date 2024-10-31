// https://www.reddit.com/r/rust/comments/kkap4e/how_to_cast_a_boxdyn_mytrait_to_an_actual_struct/
// https://www.reddit.com/r/learnrust/comments/uj68qn/how_do_i_use_downcast_ref_correctly/

// 0. all data should be a Data, Datas are updated in runners, runners are stored in trees, trees can contain handles to other trees
// 1. do not compose Datas, simple add a new Data for that specific state
// 2. all Datas are stored in btrees or as singletons
// 3. runners should declare all their existing and future Data set types at the start, even if empty for a while
// 4. only manipulate the tree when changing the structure of the program, not anything to do with data

// TODO
// 1. tree manipulations should be fallible since those fail at the start
// 2. get threading working
// 3. express runner completion dependencies via Data data, but make it simpler to do.

pub use nucleus::*;
pub use runner::*;
pub use function::*;
pub use data_set::*;
pub use data_singleton::*;
pub use data_trait::*;

// fn main() {
//     let root_data = Arc::new(Mutex::new(RootData {
//         name: "root".to_string()
//     }));

//     let mut primary = Tree::new("root", root_data, None);

//     primary.add_runner::<MovementRunner>();

//     primary.run();
// }
