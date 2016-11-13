use std::mem;

use common::util;
use world::resource::Resource;
use machine::replicator::Replicator;

#[derive(Debug)]
pub struct Space {
    blocks: Vec<SpaceBlock>,
}

#[derive(Debug)]
pub struct SpaceBlock {
    position: util::Position,
    resources: Vec<Resource>,
    replicators: Vec<Replicator>,
}

impl Space {
    pub fn new(scale: u32) -> Space {
        let mut space = Space{blocks: Vec::new()};

        let max_x = 10 * scale as i32;
        let max_y = 10 * scale as i32;
        let max_pos = util::Position::new(max_x, max_y);

        space.add_space(max_pos);

        space
    }

    pub fn get_blocks(&mut self) -> &mut Vec<SpaceBlock> {
        &mut self.blocks
    }

    pub fn add_random_space(&mut self) {
        // Find out the starting position
        let mut pos = util::Position::new(0, 0);
        if let Some(block) = self.blocks.last() {
            pos = block.position.clone();
        }

        // Random of 10 blocks/dimension
        let rand_x = pos.get_x() + util::new_random(1, 11) as i32;
        let rand_y = pos.get_y() + util::new_random(1, 11) as i32;
        let max_pos = util::Position::new(rand_x, rand_y);
        self.add_space(max_pos);
    }

    fn add_space(&mut self, max_pos: util::Position) {
        for i in 0..max_pos.get_x() {
            for j in 0..max_pos.get_y() {
                let mut resources: Vec<Resource> = Vec::new();
                for _ in 0..3 {
                    let mut res = Resource::new_random(2);
                    resources.push(res);
                }
                let pos = util::Position::new(i, j);
                let block = SpaceBlock{
                    position: pos,
                    resources: resources,
                    replicators: Vec::new(),
                };
                self.blocks.push(block);
            }
        }
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
