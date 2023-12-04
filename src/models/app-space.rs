struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    // dữ liệu của SeaCreature nằm ở trên stack
    let ferris = SeaCreature {
        // String cũng ở trên stack,
        // nhưng giữ một tham chiếu đến dữ liệu trên heap
        animal_type: String::from("con cua"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("móng vuốt"),
    };

    let sarah = SeaCreature {
        animal_type: String::from("con bạch tuộc"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("não"),
    };
    
    println!(
        "{} là một {}. Chúng có {} tay, {} chân, và có cả {}",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );
    println!(
        "{} là một {}. Chúng có {} tay, và {} legs. Và chúng không có vũ khí...",
        sarah.name, sarah.animal_type, sarah.arms, sarah.legs
    );
}
