pub use system::{Sys,SysMan,SysApply};
pub use entity::{Entity};
pub use component::{Comp};
pub use composite::{Composite};
pub use ces::{CES};
pub use systems::{Systems,Comm};

use std::sync::{Arc,RwLock};

pub mod entity;
pub mod composite;
pub mod component;
pub mod ces;
pub mod system;
pub mod systems;


//todo: convert to struct?
pub type Eid = (usize,u64); //entity specific ID


// for use in eventual arrays
pub const MAX_ENT: usize = 65535;


pub type Ents = Arc<RwLock<Vec<RwLock<Entity>>>>;
