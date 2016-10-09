extern crate vnewman;

use vnewman::world::resource::Resource;
use vnewman::machine::replicator::Replicator;

fn main() {
    let mut replicators: Vec<Replicator> = Vec::new();
    for _ in 0..100 {
        let mut replicator = Replicator::new(0);
        for _ in 0..4 {
            let res = Resource::new_random(6);
            replicator.add_resource(res);
        }
        replicators.push(replicator);
    }

    println!("Replicators before replication: {}", replicators.len());

    let mut new_replicators: Vec<Replicator> = Vec::new();
    for replicator in &mut replicators {
        if let Some(replicated) = replicator.replicate() {
            new_replicators.push(replicated);
        }
    }
    replicators.extend(new_replicators);

    println!("Replicators after replication: {}", replicators.len());
}
