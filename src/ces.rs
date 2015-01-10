use super::{Sys,SysMan,Entity,Eid,Comm, Comp};
use std::thread::Thread;
use std::sync::Arc;

//#[deriving(Show)]
pub struct CES {
    ent: Vec<u64>, 
    sys: Vec<Sys>, //immutable, and Arc for systems to lookup 
    //todo: add a hashmap/vec of sys, based on ID for faster lookups
    empty: Vec<usize>, //marked as removed/available entity slots, uint since that is based on vec total size

}


impl CES {
    pub fn new (mut s:Vec<(Sys,SysMan)>) -> CES {
        let mut vs = Vec::new();
        let mut vsm = Vec::new();
        s.into_iter().map(|(sys,sysman)| {vs.push(sys); vsm.push(sysman)}); 

        for sysman in vsm.drain() {
            let vs_ = vs.clone();
            Thread::spawn(move || {
                sysman.updater(vs_); //constantly listens to CES communication
            });
        }
        
        CES { ent: Vec::new(), 
              sys: vs,
              empty: Vec::new() }
    }

    /// update systems with matching component
    // todo: turn to trait for sys access
    pub fn update_sys<F> (&self, c: &Comp, f: F) where F: Fn(&Sys) {
        for sys in self.sys.iter() {
            'this_sys: for syscomp in sys.get_comps().iter() {
                if syscomp.is(c) {
                    f(sys);
                    break 'this_sys;
                }
            }
        }
    }

    /// broadcast to systems
    pub fn broadcast_sys (&self, comm:Comm) {
        for sys in self.sys.iter() {
            sys.update(comm.clone());
        }
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
            'this_sys: for syscomp in sys.get_comps().iter() {
                for entcomp in e.get_comps().iter() {
                    if syscomp.is(entcomp) {
                        
                       // let mut comps = Vec::new();
                        //comps.push_all(e.get_comps());
                        sys.update(Comm::AddEnt(e.clone())); //eid,comps));
                        break 'this_sys;
                    }
                }
            }
        }

        eid
    }

    pub fn rem_ent (&mut self, eid: Eid) {
        if self.ent[eid.0] == eid.1 { 
            self.empty.push(eid.0); //mark idx as empty, by adding it to empty vec
            self.broadcast_sys(Comm::RemoveEnt(eid));
        }
    }

    pub fn ent_rem_comp (&mut self, eid: Eid, c: Comp) {
        self.update_sys(&c, |sys:&Sys| sys.update(Comm::RemoveComp(eid,c)));
    }

    pub fn ent_add_comp (&mut self, eid: Eid, c: Comp) {
        self.update_sys(&c, |sys:&Sys| sys.update(Comm::AddComp(eid,c)));
    }

    pub fn shutdown(&self, s:&'static str) {
        self.broadcast_sys(Comm::Shutdown(s.to_string()));
    }
}
