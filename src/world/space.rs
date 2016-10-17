use world::resource::Resource;
use machine::replicator::Replicator;

pub struct Space {
    blocks: Vec<SpaceBlock>,
}

pub struct SpaceBlock {
    x: i32,
    y: i32,
    resources: Vec<Resource>,
    replicators: Vec<Replicator>,
}

impl Space {
    pub fn new(scale: u32) -> Space {
        let mut blocks: Vec<SpaceBlock> = Vec::new();
        for i in 0..(10 * scale as i32) {
            for j in 0..(10 * scale as i32) {
                let mut resources: Vec<Resource> = Vec::new();
                for _ in 0..3 {
                    let mut res = Resource::new_random(2);
                    resources.push(res);
                }

                let block = SpaceBlock{
                    x: i,
                    y: j,
                    resources: resources,
                    replicators: Vec::new(),
                };
                blocks.push(block);
            }
        }
        Space{blocks: blocks}
    }
}

impl SpaceBlock {
    pub fn get_resources(&self) -> &Vec<Resource> {
        &self.resources
    }

    pub fn collect_resources(&self, indexes: Vec<i32>) -> Vec<Resource> {
        let mut collected: Vec<Resource> = Vec::new();
        for index in indexes {
            let res = &self.resources[index as usize];
            collected.push(*res);
            let mut resources = self.resources;
            resources.remove(index as usize);
        }
        collected
    }
}
