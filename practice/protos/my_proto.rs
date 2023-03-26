use rust_person_proto::Person;

fn main() {
    let mut cool = Person::default();
    cool.name = String::from("yes");
    cool.height = 24;
    println!("pussy {:?}", cool);
}
