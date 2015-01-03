use super::{Entity};

pub trait Composite {
    fn build (&self) -> Entity;
}

