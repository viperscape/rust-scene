use super::{Comp,Eid, Entity};
use std::sync::mpsc::{Sender, Receiver, channel};


#[derive(Show,Clone)]
pub enum Comm {
    Msg(String),
    AddEnt(Eid,Vec<Comp>),
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
    pub fn new (c:Vec<Comp>, f: |Comm|: 'static+Send) -> (Sys,SysMan) {
        let (chs,chr) = channel();
        (Sys { comps: c, ch: chs },
         SysMan::new(chr,f))
    }
    pub fn update (&self, c: Comm) {
        println!("sending update: {}", c);
        self.ch.send(c);
    }
    pub fn get_comps (&self) -> &[Comp] {
        self.comps.as_slice()
    }
}

pub struct SysMan {
    ent: Vec<Entity>,
    ch: Receiver<Comm>,
    work: |Comm|:'static+Send,
}
impl SysMan {
    pub fn new (chr: Receiver<Comm>, f: |Comm|:'static+Send) -> SysMan {
        SysMan { ent: Vec::new(), ch: chr, work: f }
    }

    pub fn updater (mut self) {
        for comm in self.ch.iter() {
            println!("updater: {}",comm);
            match comm {
                Comm::Shutdown(r) => {
                    println!("shutting down sys: {}",r);
                    break;
                },
                _ => (self.work)(comm),
            }
        }
    }
}
