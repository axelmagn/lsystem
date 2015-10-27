extern crate lsystem;

use lsystem::*;

fn main() {
    let mut rules: MapRules<char> = MapRules::new();
    rules.set_str('A', "AB");
    rules.set_str('B', "A");
    let axiom = "A".chars().collect();
    println!("{}", show(&axiom));
    let mut system = LSystem::new(rules, axiom);
    for i in 1..10 {
        let out = system.next().unwrap();
        println!("{:3} ({:5})-> {}", i, out.len(), show(&out));
    }
}
