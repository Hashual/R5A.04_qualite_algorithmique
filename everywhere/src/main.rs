use std::io::{self,prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    
    let mut trips = input.lines();

    
    let line = trips.next().expect("msg");
    let int :i32 = line.parse::<i32>().expect("Entier");

    for _ in 0..int{
        let mut destinations : Vec<&str> = Vec::new();
        let number = trips.next().expect("msg");
        let int :i32 = number.parse::<i32>().expect("Entier");

        for _ in 0..int{
            let trip  = trips.next().expect("msg");
            if destinations.contains(&trip){
                continue;
            }
            else{
                destinations.push(trip);
            }
        }
        println!("{}",destinations.len())
    }


}


