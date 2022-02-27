use std::io::stdin;
use std::fmt;

#[derive(Debug)]
struct Ballot {
    value: i32,
    quantity: i16
}

impl Ballot {
    fn new(value: i32, quantity: i16) -> Self {
        Self { value, quantity }
    }

    fn increase_quantity(&mut self) {
        self.quantity += 1;
    }

    fn get_value(&self) -> i32 {
        self.value
    }

    fn get_quantity(&self) -> i16 {
        self.quantity
    }
}

impl fmt::Display for Ballot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} nota(s) de R$ {},00", self.get_quantity(), self.get_value())
    }
}

fn main() {
    let mut x = String::new();

    stdin().read_line(&mut x).expect("Entrada incorreta!");

    let mut draw: i32 = x.trim().parse().unwrap();

    println!("{}", draw);

    let ballots = count_possible_ballot(&mut draw);

    for ballot in ballots {
        println!("{}", ballot);
    }
}

fn count_possible_ballot(draw: &mut i32) -> Vec<Ballot> {
    let mut ballot_vec: Vec<Ballot> = vec![
        Ballot::new(100, 0),
        Ballot::new(50, 0),
        Ballot::new(20, 0),
        Ballot::new(10, 0),
        Ballot::new(5, 0),
        Ballot::new(2, 0),
        Ballot::new(1, 0)
    ];

    while *draw != 0 {
        for ballot in &mut ballot_vec {
            let new_draw = *draw - ballot.get_value();

            if new_draw >= 0 {
                *draw = new_draw;
                ballot.increase_quantity();
                break;
            }
        }
    }

    ballot_vec
}