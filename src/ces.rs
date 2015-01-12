use super::{Sys,SysMan,Entity,Eid,Comm, Comp, MAX_ENT};
use std::thread::Thread;
use std::sync::{Arc,RwLock};
use std::sync::mpsc::{Receiver};

pub type Ents = Arc<RwLock<Vec<RwLock<Entity>>>>;

//#[deriving(Show)]
pub struct CES {
    ent: Ents, //;MAX_ENT]>, 
    sys: Vec<Sys>, //immutable, for systems to lookup 
    //todo: add a hashmap/vec of sys, based on ID for faster lookups
    empty: Vec<usize>, //marked as removed/available entity slots
}

impl CES {
    pub fn new (mut s:Vec<(Sys,Receiver<Comm>)>) -> CES {
        let mut vs = Vec::new();
        let mut vschr = Vec::new();
        s.into_iter().map(|(sys,chr)| {vs.push(sys); vschr.push(chr)}); 
        
        let ces = CES { ent: Arc::new(RwLock::new(Vec::new())), 
                        sys: vs,
                        empty: Vec::new(), };

        

        vschr.into_iter().map(|chr| {
            let ents = ces.ent.clone();
            Thread::spawn(move || {
                let sysman = SysMan::new(chr, ents);
                sysman.updater(); //constantly listens to CES communication
            });
        });
  
       /* let mut ea = [RwLock::new(Entity::new(Vec::new()));MAX_ENT];
        for n in range(1,MAX_ENT) {
            ea[n] = RwLock::new(Entity::new(Vec::new()));
        }*/

        ces
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
        let e2 = e.clone();
        let eid = match self.empty.pop() {
            Some(idx) => {
                let eid = (idx,uid);
                {let mut inner = self.ent.write().unwrap();
                 *inner[idx].write().unwrap() = e2;} //swap out ent 
                eid
            }
            None => { 
                 let idx = {let mut inner = self.ent.write().unwrap();
                            inner.push(RwLock::new(e2));
                            inner.len()-1};
                (idx,uid)
            }
        };

        for sys in self.sys.iter() {
            'this_sys: for syscomp in sys.get_comps().iter() {
                for entcomp in e.get_comps().iter() {
                    if syscomp.is(entcomp) {
                        sys.update(Comm::AddEnt(eid));
                        break 'this_sys;
                    }
                }
            }
        }

        eid
    }

    fn with_ent_mut<F1> (&mut self, eid:Eid, f: F1) where F1: Fn(&mut Entity) {
        let inner = self.ent.read().unwrap();
        let mut ent = inner[eid.0].write().unwrap();
        (f)(&mut *ent);
    }

    pub fn rem_ent (&mut self, eid: Eid) {
        let flag = {let inner = self.ent.read().unwrap();
                    if inner[eid.0].read().unwrap().get_id() == eid.1 { true }
                    else { false }};

        if flag {
            self.empty.push(eid.0); //mark idx as empty, by adding it to empty vec
            self.broadcast_sys(Comm::RemoveEnt(eid));
        }
    }

    pub fn ent_rem_comp (&mut self, eid: Eid, c: Comp) {
        self.with_ent_mut(eid, |&:mut e| e.rem_comp(c));
    }

    pub fn ent_add_comp (&mut self, eid: Eid, c: Comp) {
        self.with_ent_mut(eid, |&:mut e| e.add_comp(c));
    }

    pub fn shutdown(&self, s:&'static str) {
        self.broadcast_sys(Comm::Shutdown(s.to_string()));
    }
}
