pub use system::{Sys,SysMan, Comm};
pub use entity::{Entity};
pub use component::{Comp};
pub use composite::{Composite};
pub use ces::{CES};

pub mod entity;
pub mod composite;
pub mod component;
pub mod ces;
pub mod system;


//todo: convert to tuple-struct?
pub type Eid = (uint,u64); //entity specific ID

// for use in eventual arrays
//pub const MAX_ENT: uint = 64000;
