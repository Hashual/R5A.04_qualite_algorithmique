use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());
    // Attribut les valeurs de la même ligne aux variables

    let (left, right): (usize, usize) = (iter.next().unwrap(), iter.next().unwrap());
    //  récupère le nombre de concurrents et de problèmes en faisant .next sur les lignes
    
    match (left, right) {
        (0, 0) => println!("Not a moose"),
        (l, r) if l == r && (r+l) % 2 == 0 => println!("Even {}", r+l),
        (l, r) if l < r => println!("Odd {}", r*2),
        (l, r) if l > r => println!("Odd {}", l*2),
        _ => unreachable!(),
    }

}
