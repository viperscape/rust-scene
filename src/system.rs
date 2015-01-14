use super::{CES, Systems,Comm, Comp,Eid, Ents, Entity, MAX_ENT};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::{Arc,RwLock};

#[derive(Clone)]
pub struct Sys {
    comps: Vec<Comp>, 
    ch: Sender<Comm>,
}
/// system callbacks during cycle
//todo: use trait fn and use with_ent in sysman if possible
pub struct SysApply<Ft,Fu> 
    where Ft: Fn(&mut Vec<Entity>), Fu: Fn(Eid,Comp) {
        tick: Ft,
        update: Fu,
}

impl Sys {
    pub fn new (c:Vec<Comp>) -> (Sys,Receiver<Comm>) {
        let (chs,chr) = channel();
        (Sys { comps: c, ch: chs }, chr)
    }
    pub fn update (&self, c: Comm) {
        self.ch.send(c);
    }
    pub fn get_comps (&self) -> &[Comp] {
        self.comps.as_slice()
    }
}

pub struct SysMan { //<F> 
  //  where F: Fn(&mut Vec<Entity>){
    eid: Vec<Eid>,
    ch: Receiver<Comm>,
    ent: Ents,
}
impl SysMan {
    pub fn new (chr: Receiver<Comm>, ents: Ents) -> SysMan {
        SysMan { eid: Vec::new(), ch: chr, ent: ents }
    }

    fn with_ent_mut<F1> (&mut self, eid:Eid, f: F1) where F1: Fn(&mut Entity) {
        let inner = self.ent.read().unwrap();
        let mut ent = inner[eid.0].write().unwrap();
        (f)(&mut *ent);
    }

    /// signal other sys that are interested in similar comps
    fn signal_others (&self, eid: Eid, c: &Comp, vs: &Vec<Sys>) {
        for sys in vs.iter() {
            'this_sys: for syscomp in sys.get_comps().iter() {
                if syscomp.is(c) {
                    sys.update(Comm::Update(eid,c.clone()));
                    break 'this_sys;
                }
            }
        }
    }

    // called from CES
    pub fn updater (mut self) {
        let mut chr = self.ch.recv();
        while chr.is_ok() {
            let comm = chr.unwrap();
            match comm {
                //Comm::Tick => (self.work.tick)(&mut self.ent),

                Comm::AddEnt(eid) => { //todo: consider impl as trait, similar to ces add_ent fn
                    self.eid.push(eid);
                },

                Comm::RemoveEnt(eid) => { //todo: reimpl as fixed array, with inclusion indices
                    let mut idx = 0;
                    for e in self.eid.iter() {
                        if e.1 == eid.1 { break; }
                        idx += 1;
                    }

                    self.eid.remove(idx);
                },
                Comm::Shutdown(r) => {
                    println!("shutting down sys: {}",r);
                    break;
                },
              
                _ => (),
            }

            chr = self.ch.recv();
        }
    }
}
