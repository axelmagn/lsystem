use std::collections::HashMap;
use lsystem::{LRules};

pub struct LStringRule {
    productions: HashMap<char, Vec<char>>,
}

impl LStringRule {
    pub fn new() -> LStringRule {
        LStringRule {
            productions: HashMap::new(),
        }
    }

    pub fn set(&mut self, k: char, v: Vec<char>) {
        self.productions.insert(k, v);
    }

    pub fn set_str(&mut self, k: char, v: &str) {
        let mut rule = Vec::new();
        for c in v.chars() {
            rule.push(c);
        }
        self.productions.insert(k, rule);
    }
}

impl LRules<char> for LStringRule {
    fn map(&self, k: &char) -> Option<Vec<char>> {
        match self.productions.get(k) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}

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
    use lsystem::{LSystem};

    #[test]
    fn test_algae_str() {
        let mut rules = LStringRule::new();
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
        let mut rules = LStringRule::new();
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
