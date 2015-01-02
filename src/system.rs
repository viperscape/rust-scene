use super::{Comp,Eid};
use std::comm::{Sender, Receiver, channel};


#[deriving(Show)]
pub enum Comm {
    Msg(String),
    Update(Eid,Comp),
    RemoveComp(Eid,Comp),
    RemoveEnt(Eid),
}

//#[deriving(Show)]
pub struct Sys {
    comps: Vec<Comp>, 
    ch: (Sender<Comm>, Receiver<Comm>),
}
impl Sys {
    pub fn new (c:Vec<Comp>) -> Sys {
        Sys { comps: c, ch: channel() }
    }
    pub fn update (&self, c: Comm) {
        self.ch.0.send(c);
    }
    pub fn get_comps (&self) -> &[Comp] {
        self.comps.as_slice()
    }
}
