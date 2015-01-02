extern crate "rust-scene" as scene;
use scene::{Comp,Composite,Entity,Sys,CES};

enum Composites {
    Player,
    Enemy,
}

impl Composite for Composites {
    fn build (&self) -> Entity {
        match *self {
            Composites::Player => 
                Entity::new(vec!(Comp::Health(100),
                            Comp::Collision)),
            Composites::Enemy => 
                Entity::new(vec!(Comp::Health(500),
                            Comp::Collision)),
        }
    }
}

fn main() {
  
    let mut guy = Composites::Player.build();

    println!("{}",guy);

    let mut world = CES::new();

    let health = world.register(Sys::new(vec!(Comp::Health(0))));

    let guy_id = world.add_ent(guy);

    let enemy = world.add_ent(Composites::Enemy.build());
    world.rem_ent(enemy);
}
