extern crate rand;

use self::rand::Rng;
use std::fmt;

pub struct Resource {
    iron: i32,
    silicon: i32,
    titanium: i32,
}

impl Resource {
    pub fn new_random(factor: i32) -> Resource {
        let mut thread_rng = rand::thread_rng();
        let iron = thread_rng.gen_range(0, 71 * factor);
        let silicon = thread_rng.gen_range(0, 251 * factor);
        let titanium = thread_rng.gen_range(0, 66 * factor);
        Resource{iron: iron, silicon:silicon, titanium: titanium}
    }

    pub fn get_iron(&self) -> i32 {
        self.iron
    }

    pub fn get_silicon(&self) -> i32 {
        self.silicon
    }

    pub fn get_titanium(&self) -> i32 {
        self.titanium
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Resource: [iron: {}], [silicon: {}], [titanium: {}]",
            self.iron, self.silicon, self.titanium)
    }
}
