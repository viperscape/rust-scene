use super::{Comp};
use std::rand;
use std::sync::RwLock;


//#[derive(Show,Clone)]//, PartialEq)]
pub struct Entity(Vec<RwLock<Comp>>,u64);

impl Entity {
    pub fn new (mut c: Vec<Comp>) -> Entity {
        let mut vc: Vec<RwLock<Comp>> = Vec::new();
        for n in c.drain() {
            vc.push(RwLock::new(n));
        }

        Entity(vc,rand::random::<u64>())
    }

    pub fn get_id (&self) -> u64 {
        self.1
    }

    pub fn get_comps (&self) -> &[RwLock<Comp>] {
        self.0.as_slice()
    }

    pub fn add_comp (&mut self, c:Comp) {
        self.0.push(RwLock::new(c));
    }

    //todo: make this pretty
    pub fn rem_comp (&mut self, c:Comp) {
        let mut matched = false;
        let mut idx = 0;
        for myc in self.0.iter() {
            let rl = myc.read().unwrap();
            if rl.is(&c) { matched=true; break; }
            idx += 1;
        }

        if matched { self.0.remove(idx); }
    }

    pub fn update_comp (&mut self, c:Comp) {
        for myc in self.0.iter_mut() {
            let mut wl = myc.write().unwrap();
            if wl.is(&c) { 
                *wl = c;
            }
        }
    }
}

