use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut it = input.split_whitespace();

    let n: usize = it.next().expect("Nb events manquant").parse().expect("Entier nb events");

    let mut days = [false; 366];

    for _ in 0..n {
        let a: usize = it.next().expect("Début manquant").parse().expect("Entier begin");
        let b: usize = it.next().expect("Fin manquante").parse().expect("Entier end");
        // S'assurer que les bornes restent dans 1..=365
        let start = a.clamp(1, 365);
        let end = b.clamp(1, 365);
        for d in start..=end {
            days[d] = true;
        }
    }

    let count = days.iter().filter(|&&v| v).count();
    println!("{}", count);
}
