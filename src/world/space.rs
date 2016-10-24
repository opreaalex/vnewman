use std::mem;

use world::resource::Resource;
use machine::replicator::Replicator;

#[derive(Debug)]
pub struct Space {
    blocks: Vec<SpaceBlock>,
}

#[derive(Debug)]
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

    pub fn get_blocks(&mut self) -> &mut Vec<SpaceBlock> {
        &mut self.blocks
    }
}

impl SpaceBlock {
    pub fn get_resources(&mut self) -> &mut Vec<Resource> {
        &mut self.resources
    }

    pub fn set_resources(&mut self, res: Vec<Resource>) {
        self.resources = res;
    }

    pub fn collect_resources(&mut self) -> Vec<Resource> {
        mem::replace(&mut self.get_resources(), Vec::new())
    }
}
