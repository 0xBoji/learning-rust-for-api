#![allow(dead_code)] // this line prevents compiler warnings
enum Species {
    Crab,
    Octopus,
    Fish,
    Clam
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };
    let octopus = SeaCreature {
        species: Species::Octopus,
        name: String::from("Octopus"),
        arms: 2,
        legs: 4,
        weapon: String::from("no thing"),
    };


    match ferris.species {
        Species::Crab => println!("{} là một con cua",ferris.name),
        Species::Octopus => println!("{} là một con bạch tuộc",ferris.name),
        Species::Fish => println!("{} là một con cá",ferris.name),
        Species::Clam => println!("{} là một con hến",ferris.name),
    }

    match octopus.species {
        Species::Crab => println!("{} là một con cua",octopus.name),
        Species::Octopus => println!("{} là một con bạch tuộc",octopus.name),
        Species::Fish => println!("{} là một con cá",octopus.name),
        Species::Clam => println!("{} là một con hến",octopus.name),
    }
}
