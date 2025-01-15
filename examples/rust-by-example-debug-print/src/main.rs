#![allow(unused)]

// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    let months = 12;
    println!("{} {:?} months in a year.", months, months);

    println!("{1} {0} is the {actor} name.",
             "Slater",
             "Christian",
             actor="actor's");
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    let s = Structure(3);
    //println!("Now {} {:?} will print!", s, s);
    println!("Now {:?} will print!", s);

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    let d = Deep(Structure(7));
    println!("Now {:?} will print!", d);

    println!("Now {:#?} will print!", d);

    dbg!(d);

    show_peter();
}



#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn show_peter() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}