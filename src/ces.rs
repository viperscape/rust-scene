use super::{Sys,Entity,Eid};

//#[deriving(Show)]
pub struct CES {
    ent: Vec<Entity>, 
    sys: Vec<Sys>,
    empty: Vec<uint>, //marked as removed/available entity slots
}

impl CES {
    pub fn new () -> CES {
        CES { ent: Vec::new(), sys: Vec::new(), empty: Vec::new() }
    }

    /// use rand u64 for uid.. for now,; but, consider switching to incremental u64
    pub fn add_ent (&mut self, e:Entity) -> Eid {
        let uid = e.get_id(); //rand style entity uid comes from entity build, copy it
        let empty = self.empty.pop();
        let eid = match empty {
            Some(idx) => {
                let eid = (idx,uid);
                self.ent[idx] = e; //swap out ent 
                eid
            }
            None => { 
                self.ent.push(e);
                (self.ent.len()-1,uid)
            }
        };

        // todo: signal systems of change
        for sys in self.sys.iter() {
            for syscomp in sys.get_comps().iter() {
                for entcomp in self.ent[eid.0].get_comps().iter() {
                    if syscomp.is(entcomp) {
                        println!("Comp: {}",syscomp);
                    }
                // sys.update(Comm::Update(eid,
                }
            }
        }

        eid
         
    }
    pub fn rem_ent (&mut self, e: Eid) {
        if self.ent[e.0].get_id() == e.1 { 
            self.empty.push(e.0);

            //todo: consider clearing out entity? probably not necessary if skipped over anyways
            //self.ent[e.0].0 = Vec::new();
            //self.ent[e.0].1 = 0;
        }
    }

    pub fn register (&mut self, s:Sys) -> uint {
        self.sys.push(s);
        self.sys.len()-1
    }
}
