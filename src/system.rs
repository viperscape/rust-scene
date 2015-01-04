use super::{Comp,Eid, Entity};
use std::sync::mpsc::{Sender, Receiver, channel,RecvError};


#[derive(Show)]
pub enum Comm {
    Msg(String),
    AddEnt(Eid,Vec<Comp>),
    AddComp(Eid,Comp), //ent add comp 
    Update(Eid,Comp),
    RemoveComp(Eid,Comp), //ent remove comp
    RemoveEnt(Eid),
}

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

pub struct SysMan {
    ent: Vec<Entity>,
    ch: Receiver<Comm>,
}
impl SysMan {
    pub fn new (e:Vec<Entity>, chr: Receiver<Comm>) -> SysMan {
        SysMan { ent: e, ch: chr }
    }
    pub fn update (&self) -> Result<Comm,RecvError> {
        self.ch.recv()
    }
}
