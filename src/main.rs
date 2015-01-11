#![feature(box_syntax)]

extern crate "rust-scene" as scene;

use scene::{Comp,Composite,Entity,Sys,SysApply,CES};

use std::thread::Thread;
use std::sync::{Arc,RwLock,RwLockWriteGuard};
use std::sync::mpsc::{Sender, Receiver, channel};

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


    let mut e = Ent { comps: RwLock::new(vec!()),
                      uid: RwLock::new(1), };

    let mut es = [e];

    let aes =  Arc::new(es);
    let aes2 = aes.clone();
    
    let (chs,chr) = channel();

    Thread::spawn(move || {
        let mut inner = aes2[0].uid.write().unwrap();
         *inner = 2u32;
        let mut inner = &mut aes2[0].comps.write().unwrap();
         inner.push(Comp::Collision);

        
        chs.send(true);
    });
    

    chr.recv();
    assert_eq!(*aes[0].uid.read().unwrap(), 2);
    assert_eq!(aes[0].comps.read().unwrap()[0], Comp::Collision); 
}

struct Ent {
    comps: RwLock<Vec<Comp>>,
    uid: RwLock<u32>,
}

struct Ents {
    ents: Arc<[Ent;1]>,
    empty: Vec<u32>,
}
