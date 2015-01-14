use super::{Entity,Eid,Comp,Ents};

pub trait Systems {
    fn with_ent_mut<F1> (ent: &mut Ents, eid:Eid, f: F1) where F1: Fn(&mut Entity) {
        let inner = ent.read().unwrap();
        let mut ent = inner[eid.0].write().unwrap();
        (f)(&mut *ent);
    }
}

/// communication from CES to systems, and between systems
#[derive(Show,Clone)]
pub enum Comm {
    AddEnt(Eid),
    AddComp(Eid,Comp), //ent add comp 
    Update(Eid,Comp),
    
    RemoveComp(Eid,Comp), //ent remove comp
    RemoveEnt(Eid),

    Shutdown(String),
    Tick, //render tick, triggers next cycle for system
}
