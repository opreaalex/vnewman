extern crate vnewman;

// use vnewman::common::util::Position;
// use vnewman::world::resource::Resource;
use vnewman::world::space::Space;
// use vnewman::machine::replicator::Replicator;

fn main() {
    let mut space = Space::new(1);
    println!("{:?}", space);

    space.add_random_space();
    println!("{:?}", space);

    space.add_random_space();
    println!("{:?}", space);
}
