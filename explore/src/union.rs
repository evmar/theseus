use std::collections::HashMap;

use crate::ast::Var;

#[derive(Default)]
pub struct Union<'a> {
    map: HashMap<&'a Var, &'a Var>,
}

impl<'a> Union<'a> {
    pub fn insert(&mut self, v: &'a Var) {
        if !self.map.contains_key(v) {
            self.map.insert(v, v);
        }
    }

    pub fn join(&mut self, v1: &'a Var, v2: &'a Var) {
        let v1 = self.find(v1);
        let v2 = self.find(v2);
        if v1 < v2 {
            self.map.insert(v2, v1);
        } else if v1 > v2 {
            self.map.insert(v1, v2);
        }
    }

    pub fn find(&self, v: &'a Var) -> &'a Var {
        let mut v = v;
        loop {
            let next = self.map.get(v).unwrap();
            if *next == v {
                return next;
            }
            v = next;
        }
    }

    pub fn sets(&self) -> Vec<Vec<&'a Var>> {
        let mut sets: HashMap<&Var, Vec<&Var>> = HashMap::new();
        for (v, u) in self.map.iter() {
            let u = self.find(u);
            sets.entry(u).or_default().push(v);
        }
        sets.into_values().collect()
    }
}
