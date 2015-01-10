#[derive(Show, PartialEq, Copy, Clone)] // copy exists bc of CES sending systems updates
pub enum Comp {
    Collect,
    Health(u8),
    Collision,
}

impl Comp {
    pub fn is(&self, other: &Comp) -> bool {
        match (self, other) {
            (&Comp::Health(_), &Comp::Health(_)) => true,
            _ => self == other, //use for flags (non-data binding enums)
        }
    }
}
