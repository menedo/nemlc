use poirot::raster::Lattice;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct NeiEndpoint {
    pub index: usize,
    pub name: Option<String>,
    pub neighbor: Option<Rc<RefCell<Vec<NeiEndpoint>>>>,
}

impl NeiEndpoint {
    pub fn new(index: usize, name: Option<String>) -> Self {
        NeiEndpoint {
            index: index,
            name: name,
            neighbor: None,
        }
    }

    pub fn attach(&mut self, sub_end: NeiEndpoint) {
        match self.neighbor {
            Some(ref x) => {
                let nei = &mut x.borrow_mut();
                nei.push(sub_end);
            }
            None => {
                let mut nei = Vec::new();
                nei.push(sub_end);
                self.neighbor = Some(Rc::new(RefCell::new(nei)));
            }
        }
    }

    pub fn create_object_tree(&self) -> Lattice {
        let mut name = "none".to_string();
        match &self.name {
            Some(n) => {
                name = n.to_string();
            }
            None => {}
        }

        let mut node = Lattice::new(name);

        match &self.neighbor {
            Some(nh) => {
                let mut index = 0;
                while index < nh.borrow().len() {
                    let n = nh.borrow()[index].create_object_tree();
                    node.add_sub(n);
                    index += 1;
                }
            }
            None => {}
        }

        node
    }
}

pub struct Endpoint {
    pub name: String,
    pub root: Option<Rc<RefCell<Endpoint>>>,
    pub neighbor: Option<Rc<RefCell<HashMap<String, Endpoint>>>>,
}

impl Endpoint {
    pub fn new(input: String) -> Self {
        Endpoint {
            name: input,
            root: None,
            neighbor: None,
        }
    }

    pub fn show(&self) {
        println!("name: {}", self.name);

        match self.neighbor {
            Some(ref x) => {
                let nei = x.borrow();
                for (key, value) in &*nei {
                    println!("{} {}", key, value.name);
                }
            }
            None => {}
        }
    }

    pub fn attach(&mut self, sub: Endpoint) -> Result<(), std::io::Error> {
        match self.neighbor {
            Some(ref x) => {
                let nei = &mut x.borrow_mut();
                nei.insert(sub.name.clone(), sub);
            }
            None => {
                let mut nei = HashMap::new();
                nei.insert(sub.name.clone(), sub);
                self.neighbor = Some(Rc::new(RefCell::new(nei)));
            }
        }

        Ok(())
    }
}
