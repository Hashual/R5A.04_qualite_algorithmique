use std::io::{self, prelude::*};

fn make_status(n: i32) -> String {
    if n % 2 == 0 {
        format!("{} is even", n)
    } else {
        format!("{} is odd", n)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut lines = input.lines();
    lines.next(); // ignore first line
    for line in lines {
        let n: i32 = line.parse::<i32>().expect("Entier");
        println!("{}", make_status(n));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_status() {
        assert_eq!(make_status(2), "2 is even");
        assert_eq!(make_status(3), "3 is odd");
        assert_eq!(make_status(0), "0 is even");
        assert_eq!(make_status(-1), "-1 is odd");
        assert_eq!(make_status(-2), "-2 is even");
    }
}