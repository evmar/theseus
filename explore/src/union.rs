use std::collections::{HashMap, HashSet};

use crate::ast::Var;

#[derive(Default)]
pub struct Union {
    map: HashMap<Var, Var>,
}

impl Union {
    pub fn insert(&mut self, v: &Var) {
        if !self.map.contains_key(v) {
            self.map.insert(v.clone(), v.clone());
        }
    }

    pub fn join(&mut self, v1: &Var, v2: &Var) {
        let v1 = self.find(v1);
        let v2 = self.find(v2);
        if v1 < v2 {
            self.map.insert(v2.clone(), v1.clone());
        } else if v1 > v2 {
            self.map.insert(v1.clone(), v2.clone());
        }
    }

    pub fn find<'a>(&'a self, v: &Var) -> &'a Var {
        let mut v = v;
        loop {
            let next = self.map.get(v).unwrap();
            if next == v {
                return next;
            }
            v = next;
        }
    }

    pub fn sets(&self) -> Vec<Vec<Var>> {
        let mut sets: HashMap<&Var, HashSet<Var>> = HashMap::new();
        for (v, u) in self.map.iter() {
            let u = self.find(u);
            log::info!("{} -> {}", v, u);
            sets.entry(u).or_default().insert(v.clone());
        }
        sets.into_values()
            .map(|set| set.into_iter().collect())
            .collect()
    }
}
