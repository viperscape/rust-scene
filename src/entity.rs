use super::{Comp};
use std::rand;

#[derive(Show,Clone)]//, PartialEq)]
pub struct Entity(Vec<Comp>,u64);

impl Entity {
    pub fn new (c: Vec<Comp>) -> Entity {
        Entity(c,rand::random::<u64>())
    }

    pub fn get_id (&self) -> u64 {
        self.1
    }

    pub fn get_comps (&self) -> &[Comp] {
        self.0.as_slice()
    }

    pub fn add_comp (&mut self, c:Comp) {
        self.0.push(c);
    }

    //todo: make this pretty
    pub fn rem_comp (&mut self, c:Comp) {
        let mut matched = false;
        let mut idx = 0;
        for myc in self.0.iter() {
            if myc.is(&c) { matched=true; break; }
            idx += 1;
        }

        if matched { self.0.remove(idx); }
    }



    //ignore all of this
  /*  fn add_node (&mut self, e:Entity) {
        for i in self.0.iter_mut() {
            match *i {
                Comp::Nodes(ref mut nv) => { nv.push(e); break },
                _ => (),
            }
        }
    }*/

    /// returns vector of element positions to find node ///
    /// use pop to move cursor on vector, and get entity at that position ///
  /*  fn find_node (&self, e: &str) -> Option<Vec<u8>> {
        let mut trace = Vec::new();
        let mut res: Option<Vec<u8>> = None;
        if self.0 == e.as_slice() {
            println!("found {}",self);
            Some(trace)
        }
        else {
            for i in self.1.iter() {
                match *i {
                    Comp::Nodes(ref nv) => {
                        let mut t = 0;
                        for j in nv.iter() {
                            match j.find_node(e.as_slice()) {
                                Some(r) => {
                                    trace.push_all(r.as_slice());
                                    trace.push(t);
                                },
                                None => (),
                            }
                            t += 1;
                        }
                    },
                    _ => (),
                }
            }

            if (trace.as_slice().len()>0) {Some(trace)}
            else {None}
        }

    }*/
}

