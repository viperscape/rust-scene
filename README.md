## Rust Scene ##

This is, in its current state, template code to review and use in a project. My hope is to abstract enough to make this an actual library (though I will have to move away from using so many Enums for that to work, as in Rust you cannot overload nor extend Enums)


#### Goals: ####

* Component Entity System
* Threading support, to scale out any System with use of Event Bus or Messaging (currently messaging is planned)
* Scene graph/hierarchy for dependencies amongst entities (with Component propagation)
* Make use pre-allocated arrays for memory-continuity, for now Vecs everywhere
* Try and avoid vtable, reflection, indirection where possible

[see example](/src/main.rs)
