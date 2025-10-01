use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());
    // Attribut les valeurs de la même ligne aux variables

    let (contestants, problems): (usize, usize) = (iter.next().unwrap(), iter.next().unwrap());
    //  récupère le nombre de concurrents et de problèmes en faisant .next sur les lignes

    println!("\n{}", problems);
}
