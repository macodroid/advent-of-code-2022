use std::fs::File;
use std::io::BufRead;

fn main() {
    let filename = "calories.txt";
    let file = File::open(filename).unwrap();
    let reader  = std::io::BufReader::new(file);
    let mut max_calories: i32 = 0;
    let mut current_calories: i32 = 0;
    let mut vec = Vec::<i32>::new();

    for  line in reader.lines() {
        let line = line.unwrap();
        // Part 1
        // if line != "" {
        //     let calories: i32 = line.parse().unwrap();
        //     current_calories += calories;
        // } else {
        //     if current_calories > max_calories {
        //         max_calories = current_calories;
        //     }
        //     current_calories = 0;
        // }

        // Part 2
         if line != "" {
            let calories: i32 = line.parse().unwrap();
            current_calories += calories;
        } else {
             vec.push(current_calories);
             current_calories = 0;
        }
    }
    vec.sort();
    println!("{}", vec[vec.len() - 1]);
    println!("{}", vec[vec.len() - 2]);
    println!("{}", vec[vec.len() - 3]);
    println!("{}", vec[vec.len() - 3] + vec[vec.len() - 1] + vec[vec.len() - 2]);
}
