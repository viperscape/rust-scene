use super::{Comp,Eid, Entity, MAX_ENT};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::Arc;

/// communication from CES to systems, and between systems
#[derive(Show,Clone)]
pub enum Comm {
    Msg(String),
    AddEnt(Entity),//Eid,Vec<Comp>),
    AddComp(Eid,Comp), //ent add comp 
    Update(Eid,Comp),
    RemoveComp(Eid,Comp), //ent remove comp
    RemoveEnt(Eid),
    Shutdown(String),
}

#[derive(Clone)]
pub struct Sys {
    comps: Vec<Comp>, 
    ch: Sender<Comm>,
}
impl Sys {
    pub fn new (c:Vec<Comp>, f:Box<Fn(Comm)+Send>) -> (Sys,SysMan) {
        let (chs,chr) = channel();
        (Sys { comps: c, ch: chs },
         SysMan::new(chr,f))
    }
    pub fn update (&self, c: Comm) {
        //println!("sending update: {}", c);
        self.ch.send(c);
    }
    pub fn get_comps (&self) -> &[Comp] {
        self.comps.as_slice()
    }
}

pub struct SysMan {
    ent: Vec<Entity>,
    ch: Receiver<Comm>,
    work: Box<Fn(Comm)+'static+Send>,
}
impl SysMan {
    pub fn new (chr: Receiver<Comm>, f: Box<Fn(Comm)+Send>) -> SysMan {
        SysMan { ent: Vec::new(), ch: chr, work: f }
    }

    pub fn with_ent<F> (&mut self, eid:Eid, f: F) where F: Fn(&mut Entity) {
        for e in self.ent.iter_mut() {
            if e.get_id() == eid.1 {
                (f)(e);
            }
        }
    }

    // called from CES
    pub fn updater (mut self, avs: Arc<Vec<Sys>>) {
        let mut chr = self.ch.recv();
        while chr.is_ok() {
            let comm = chr.unwrap();
            match comm {
                Comm::Update(eid,comp) => (self.work)(comm),

                Comm::AddEnt(e) => { //todo: consider impl as trait, similar to ces add_ent fn
                    println!("adding ent: {}", e);
                    self.ent.push(e);
                },

                Comm::RemoveEnt(eid) => { //todo: reimpl as fixed array, with inclusion indices
                    println!("removing ent: {}", eid);
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
