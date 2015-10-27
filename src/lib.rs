use std::collections::HashMap;
use std::hash::Hash;

/// A type containing the full specification for an L-system.
pub struct LSystem<T, P> where P: LRules<T> {
    rules: P,
    pub axiom: Vec<T>,
    state: Vec<T>,
}

impl<T, P> LSystem<T, P> where P: LRules<T>, T: Clone {
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

impl<T, P> Iterator for LSystem<T, P> where P: LRules<T>, T: Clone {
    type Item = Vec<T>;

    /// Get the next iteration of the L-System by evaluating its associated 
    /// production rules on its current states.
    fn next(&mut self) -> Option<Vec<T>> {
        let mut i: usize = 0;
        let mut expanded = false;
        while i < self.state.len() {
            let atom = self.state[i].clone();
            let production = self.rules.map(&atom);
            match production {
                Some(atoms) => {
                    self.state.remove(i);
                    for a in atoms.into_iter() {
                        self.state.insert(i, a);
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

/// A set of production rule for an L-system, which maps an item to a list of
/// items which will replace it in the L-system state.
pub trait LRules<T> {
    /// perform a mapping of one atom to a string.
    fn map(&self, input: &T) -> Option<Vec<T>>; 
}

pub struct MapRules<T: Hash + Eq> {
    productions: HashMap<T, Vec<T>>,
}

impl<T> MapRules<T> where T: Hash + Eq {
    pub fn new() -> MapRules<T> {
        MapRules {
            productions: HashMap::new(),
        }
    }

    pub fn set(&mut self, k: T, v: Vec<T>) -> Option<Vec<T>> {
        self.productions.insert(k, v)
    }
}

impl MapRules<char> {
    pub fn set_str(&mut self, k: char, v: &str) -> Option<Vec<char>> {
        let mut rule = Vec::new();
        for c in v.chars() {
            rule.push(c);
        }
        self.set(k, rule)
    }
}

impl<T: ?Sized> LRules<T> for MapRules<T> where T: Clone + Hash + Eq {
    fn map(&self, input: &T) -> Option<Vec<T>> {
        match self.productions.get(input) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}

/// A convenience function to print out the String representation of a char
/// vector.
pub fn show(v: &Vec<char>) -> String {
    let mut out = String::with_capacity(v.len());
    for c in v.iter() {
        out.push(*c);
    }
    out
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_algae_str() {
        let mut rules: MapRules<char> = MapRules::new();
        rules.set_str('A', "AB");
        rules.set_str('B', "A");
        let axiom = "A".chars().collect();
        let mut system = LSystem::new(rules, axiom);

        let out = system.next().unwrap();
        let expected: Vec<char> = "AB".chars().collect();
        assert_eq!(expected, out);

        let out = system.next().unwrap();
        let expected: Vec<char> = "ABA".chars().collect();
        assert_eq!(expected, out);

        let out = system.next().unwrap();
        let expected: Vec<char> = "ABAAB".chars().collect();
        assert_eq!(expected, out);

        let out = system.next().unwrap();
        let expected: Vec<char> = "ABAABABA".chars().collect();
        assert_eq!(expected, out);

        let out = system.next().unwrap();
        let expected: Vec<char> = "ABAABABAABAAB".chars().collect();
        assert_eq!(expected, out);

        let out = system.next().unwrap();
        let expected: Vec<char> = "ABAABABAABAABABAABABA".chars().collect();
        assert_eq!(expected, out);
        
        let out = system.next().unwrap();
        let expected: Vec<char> = "ABAABABAABAABABAABABAABAABABAABAAB".chars().collect();
        assert_eq!(expected, out);
    }

    #[test]
    fn test_pythagoras_tree() {
        let mut rules: MapRules<char> = MapRules::new();
        rules.set_str('1', "11");
        rules.set_str('0', "1[0]0");
        let axiom = "0".chars().collect();
        let mut system = LSystem::new(rules, axiom);

        let out = system.next().unwrap();
        let expected: Vec<char> = "1[0]0".chars().collect();
        assert_eq!(expected, out);

        let out = system.next().unwrap();
        let expected: Vec<char> = "11[1[0]0]1[0]0".chars().collect();
        assert_eq!(expected, out);

        let out = system.next().unwrap();
        let expected: Vec<char> = "1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0".chars().collect();
        assert_eq!(expected, out);
    }
}
