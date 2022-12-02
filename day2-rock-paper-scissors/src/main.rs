/*
opponent (A = Rock, B = Paper, C = Scissors)
me (X = Rock, Y = Paper, Z = Scissors)
Point system:
    X(rock) 1p
    Y(paper) 2p
    Z(scissors) 3p
    -------------
    lose = 0
    win = 6
    draw = 3
*/


use std::fs::File;
use std::io::BufRead;

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader  = std::io::BufReader::new(file);
    let mut total_score = 0;
    let mut round_score = 0;
    for  line in reader.lines() {
        let line = line.unwrap();
        let mut choices = line.split_whitespace();
        // Part 1
        // round_score = match (choices.next().unwrap(), choices.next().unwrap()) {
        //     ("A", "X") => 4 ,
        //     ("A", "Y") => 8,
        //     ("A", "Z") => 3,
        //     ("B", "X") => 1,
        //     ("B", "Y") => 5,
        //     ("B", "Z") => 9,
        //     ("C", "X") => 7,
        //     ("C", "Y") => 2,
        //     ("C", "Z") => 6,
        //     _ => 0,
        // };
        // total_score += round_score;

        // Part 2 just rearrange the match output
        round_score = match (choices.next().unwrap(), choices.next().unwrap()) {
            ("A", "X") => 3 ,
            ("A", "Y") => 4,
            ("A", "Z") => 8,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 2,
            ("C", "Y") => 6,
            ("C", "Z") => 7,
            _ => 0,
        };
        total_score += round_score;
    }
    println!("{}", total_score);
}
