
use world::resource::Resource;
use world::space::{Space, SpaceBlock};

#[derive(Debug)]
pub struct Replicator {
    id: i32,
    collector: Vec<Resource>,
}

impl Replicator {
    pub fn new(id: i32) -> Replicator {
        Replicator{id: id, collector: Vec::new()}
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn replicate(&mut self) -> Option<Replicator> {
        if self.meets_requirements() {
            self.collector = Vec::new();
            Some(Replicator::new(0))
        } else {
            None
        }
    }

    pub fn add_resource(&mut self, res: Resource) {
        self.collector.push(res);
    }

    fn meets_requirements(&self) -> bool {
        let mut iron = 0;
        let mut silicon = 0;
        let mut titanium = 0;

        for res in &self.collector {
            iron += res.get_iron();
            silicon += res.get_silicon();
            titanium += res.get_titanium();
        }

        if iron >= 500 && silicon >= 1500 && titanium >= 200 {
            true
        } else {
            false
        }
    }
}
