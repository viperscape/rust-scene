#[deriving(Show, PartialEq)]
pub enum Comp {
    Collect,
    Health(int),
   // Nodes(Vec<Entity>),
    Collision,
}

impl Comp {
    pub fn is(&self, other: &Comp) -> bool {
        match (self, other) {
           // (&Comp::Collect, &Comp::Collect) => true,
            (&Comp::Health(_), &Comp::Health(_)) => true,
           // (&Comp::Collision, &Comp::Collision) => true,
            _ => self == other, //use for flags (non-data binding enums)
        }
    }
}
