    //ignore all of this
  /*  fn add_node (&mut self, e:Entity) {
        for i in self.0.iter_mut() {
            match *i {
                Comp::Nodes(ref mut nv) => { nv.push(e); break },
                _ => (),
            }
        }
    }*/

    /// returns vector of element positions to find node ///
    /// use pop to move cursor on vector, and get entity at that position ///
  /*  fn find_node (&self, e: &str) -> Option<Vec<u8>> {
        let mut trace = Vec::new();
        let mut res: Option<Vec<u8>> = None;
        if self.0 == e.as_slice() {
            println!("found {}",self);
            Some(trace)
        }
        else {
            for i in self.1.iter() {
                match *i {
                    Comp::Nodes(ref nv) => {
                        let mut t = 0;
                        for j in nv.iter() {
                            match j.find_node(e.as_slice()) {
                                Some(r) => {
                                    trace.push_all(r.as_slice());
                                    trace.push(t);
                                },
                                None => (),
                            }
                            t += 1;
                        }
                    },
                    _ => (),
                }
            }

            if (trace.as_slice().len()>0) {Some(trace)}
            else {None}
        }

    }*/
