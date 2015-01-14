extern crate "rust-scene" as scene;
use scene::{Comp,Composite,Entity,Sys,SysApply,CES};

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
                Entity::new(vec!(Comp::Health(50),
                            Comp::Collision)),
        }
    }
}

fn main() {
    let sys = vec!(Sys::new(vec!(Comp::Health(0),
                                 Comp::Collision),
                            ));//SysApply{ tick: |&:ref mut vs| (),
                                 //     update: |&:eid,comp| () } ));

    let mut world = CES::new(sys);

    let player = world.add_ent(Composites::Player.build());
    let enemy = world.add_ent(Composites::Enemy.build());

    world.rem_ent(enemy);
    world.ent_rem_comp(player,Comp::Collision);

    world.shutdown("end");

}
