use super::{Comp,Eid, Entity, MAX_ENT};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::Arc;

/// communication from CES to systems, and between systems
#[derive(Show,Clone)]
pub enum Comm {
    AddEnt(Entity),//Eid,Vec<Comp>),
    AddComp(Eid,Comp), //ent add comp 
    Update(Eid,Comp),
    
    RemoveComp(Eid,Comp), //ent remove comp
    RemoveEnt(Eid),
    Shutdown(String),

    Tick, //render tick, triggers next cycle for system
    Msg(String),
}

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
    pub fn new (c:Vec<Comp>) -> (Sys,SysMan) {
        let (chs,chr) = channel();
        (Sys { comps: c, ch: chs },
         SysMan::new(chr))
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
    ent: Vec<Entity>,
    ch: Receiver<Comm>,
  //  work:F,
}
impl SysMan {
    pub fn new (chr: Receiver<Comm>) -> SysMan {
        SysMan { ent: Vec::new(), ch: chr }
    }

    fn with_ent<F1> (&mut self, eid:Eid, f: F1) where F1: Fn(&mut Entity) {
        for e in self.ent.iter_mut() {
            if e.get_id() == eid.1 {
                (f)(e);
            }
        }
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
    pub fn updater (mut self, vs: Vec<Sys>) {
        let mut chr = self.ch.recv();
        while chr.is_ok() {
            let comm = chr.unwrap();
            match comm {
                Comm::Update(eid,comp) => {
                    self.with_ent(eid, |&:mut e| e.update_comp(comp)); //for now just swap the component out
                    // todo: consider commuting component updates, impl callback for customization
                },

                //Comm::Tick => (self.work.tick)(&mut self.ent),

                Comm::AddEnt(e) => { //todo: consider impl as trait, similar to ces add_ent fn
                    self.ent.push(e);
                },

                Comm::RemoveEnt(eid) => { //todo: reimpl as fixed array, with inclusion indices
                    let mut idx = 0;
                    for e in self.ent.iter() {
                        if e.get_id() == eid.1 { break; }
                        idx += 1;
                    }

                    self.ent.remove(idx);
                },

                Comm::AddComp(eid,comp) => {
                    self.with_ent(eid, |&:mut e| e.add_comp(comp));
                },

                Comm::RemoveComp(eid,comp) => {
                    self.with_ent(eid, |&:mut e| e.rem_comp(comp));
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
