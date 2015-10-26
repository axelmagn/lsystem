pub struct LSystem<T, P> where P: LRules<T> {
    rules: P,
    pub axiom: Vec<T>,
    state: Vec<T>,
}

impl<T, P> LSystem<T, P> where P: LRules<T>, T: Copy {
    /// create a new L-System from rules and an axiom
    pub fn new(rules: P, axiom: Vec<T>) -> LSystem<T, P> {
        LSystem {
            rules: rules,
            state: axiom.clone(),
            axiom: axiom,
        }
    }

    /// reset the L-System state back to its axiom
    pub fn reset(&mut self) {
        self.state = self.axiom.clone();
    }
}

impl<T, P> Iterator for LSystem<T, P> where P: LRules<T>, T: Copy {
    type Item = Vec<T>;

    /// mutate the l system
    fn next(&mut self) -> Option<Vec<T>> {
        let mut i: usize = 0;
        let mut expanded = false;
        while i < self.state.len() {
            let atom = self.state[i];
            let production = self.rules.map(&atom);
            match production {
                Some(atoms) => {
                    self.state.remove(i);
                    for a in atoms.iter() {
                        self.state.insert(i, *a);
                        i += 1;
                    }
                    expanded = true;
                },
                None => {
                    i += 1;
                }
            }
        }
        if expanded {
            Some(self.state.clone())
        } else {
            None
        }

    }
}

pub trait LRules<T> {
    /// perform a mapping of one atom to a string.
    fn map(&self, input: &T) -> Option<Vec<T>>;
}

