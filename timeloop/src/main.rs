use std::io::{self, prelude::*};


fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");

    let number_of_abracadabra : i32 = input.trim().parse().expect("Entier");
    for count in 0..number_of_abracadabra {
         println!("{} abracadabra", count+1);
    }
}
