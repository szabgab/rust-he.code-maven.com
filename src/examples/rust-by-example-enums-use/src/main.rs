// An attribute to hide warnings for unused code.
#![allow(dead_code)]

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
    Beginner,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Stage::{Beginner, Advanced};
    // use crate::Stage::Beginner;
    // use crate::Stage::Advanced;
    // Automatically `use` each name inside `Role`.
    use crate::Role::*;
    //use crate::Role::{Student, Teacher, Beginner};

    // Equivalent to `Stage::Beginner`.
    let stage = Beginner;
    // Equivalent to `Role::Student`.
    let role = Student;

    match stage {
        // Note the lack of scoping because of the explicit `use` above.
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        // Note again the lack of scoping.
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
        //_ => println!("default"),
        //Beginner => println!("Beginner"),
        Role::Beginner => println!("Beginner"),
    }
}