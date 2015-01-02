use super::{Comp,Eid,Entity};

pub trait Composite {
    fn build (&self) -> Entity;
}

