use super::{Sys,Entity,Eid,Comm, Comp};

//#[deriving(Show)]
pub struct CES {
    ent: Vec<u64>, 
    sys: Vec<Sys>,
    empty: Vec<uint>, //marked as removed/available entity slots
}

impl CES {
    pub fn new (s:Vec<Sys>) -> CES {
        CES { ent: Vec::new(), 
              sys: s, 
              empty: Vec::new() }
    }


    fn with_sys (&self, c: &Comp) -> Option<&Sys> {
        for sys in self.sys.iter() {
            for syscomp in sys.get_comps().iter() {
                if syscomp.is(c) {
                    return Some(sys)
                }
            }
        }
        None
    }


    /// use rand u64 for uid.. for now,; but, consider switching to incremental u64
    pub fn add_ent (&mut self, e:Entity) -> Eid {
        let uid = e.get_id(); //rand style entity uid comes from entity build, copy it
        let empty = self.empty.pop();
        let eid = match empty {
            Some(idx) => {
                let eid = (idx,uid);
                self.ent[idx] = uid; //swap out ent 
                eid
            }
            None => { 
                self.ent.push(uid);
                (self.ent.len()-1,uid)
            }
        };

         for sys in self.sys.iter() {
             let mut comps = Vec::new();
             comps.push_all(e.get_comps());
             sys.update(Comm::AddEnt(eid,comps));
         }

        eid
    }

    pub fn rem_ent (&mut self, eid: Eid) {
        if self.ent[eid.0] == eid.1 { 
            self.empty.push(eid.0); //mark idx as empty, by adding it to empty vec

            for sys in self.sys.iter() {
                sys.update(Comm::RemoveEnt(eid));
            }
        }
    }

    pub fn ent_rem_comp (&mut self, eid: Eid, c: Comp) {
       for sys in self.sys.iter() {
            for syscomp in sys.get_comps().iter() {
                if syscomp.is(&c) {
                    sys.update(Comm::RemoveComp(eid,c));
                    return;
                }
            }
        }
    }

    pub fn ent_add_comp (&mut self, eid: Eid, c: Comp) {
        for sys in self.sys.iter() {
            for syscomp in sys.get_comps().iter() {
                if syscomp.is(&c) {
                    sys.update(Comm::AddComp(eid,c));
                    return;
                }
            }
        }
    }


    //todo: remove this, register all sys at ces new
   /* pub fn register (&mut self, s:Sys) -> uint {
        self.sys.push(s);
        self.sys.len()-1
    }*/
}
