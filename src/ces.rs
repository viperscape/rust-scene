use super::{Sys,Entity,Eid,Comm, Comp};

//#[deriving(Show)]
pub struct CES {
    ent: Vec<Entity>, 
    sys: Vec<Sys>,
    empty: Vec<uint>, //marked as removed/available entity slots
}

impl CES {
    pub fn new () -> CES {
        CES { ent: Vec::new(), sys: Vec::new(), empty: Vec::new() }
    }


    fn sig_sys (&self, eid: Eid, f: |&Sys,&Comp|) {
        // todo: signal systems of change
        for sys in self.sys.iter() {
            for syscomp in sys.get_comps().iter() {
                for entcomp in self.ent[eid.0].get_comps().iter() {
                    if syscomp.is(entcomp) {
                        
                        let c = entcomp;
                        f(sys,c);

                        //sys.update(Comm::Update(eid,*c));
                    }
                }
            }
        }
}

    /// use rand u64 for uid.. for now,; but, consider switching to incremental u64
    pub fn add_ent (&mut self, e:Entity) -> Eid {
        let uid = e.get_id(); //rand style entity uid comes from entity build, copy it
        let empty = self.empty.pop();
        let eid = match empty {
            Some(idx) => {
                let eid = (idx,uid);
                self.ent[idx] = e; //swap out ent 
                eid
            }
            None => { 
                self.ent.push(e);
                (self.ent.len()-1,uid)
            }
        };

        self.sig_sys(eid, |sys:&Sys, c:&Comp| {
            sys.update(Comm::Update(eid,*c))
        });

        eid
         
    }
    pub fn rem_ent (&mut self, eid: Eid) {
        if self.ent[eid.0].get_id() == eid.1 { 
            self.empty.push(eid.0);


            self.sig_sys(eid, |sys:&Sys, _| {
                sys.update(Comm::RemoveEnt(eid))
            });

            //todo: consider clearing out entity? probably not necessary if skipped over anyways
            //self.ent[e.0].0 = Vec::new();
            //self.ent[e.0].1 = 0;
        }
    }

    pub fn ent_rem_comp (&mut self, eid: Eid, c: Comp) {
        self.sig_sys(eid, |sys:&Sys, _| {
            sys.update(Comm::RemoveComp(eid,c))
        });
        
        self.ent[eid.0].rem_comp(c);
    }

    pub fn ent_add_comp (&mut self, eid: Eid, c: Comp) {
        
        self.ent[eid.0].add_comp(c);
        self.sig_sys(eid, |sys:&Sys, _| {
            sys.update(Comm::Update(eid,c))
        });
    }

    pub fn register (&mut self, s:Sys) -> uint {
        self.sys.push(s);
        self.sys.len()-1
    }
}
