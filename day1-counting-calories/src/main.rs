use std::fs::File;
use std::io::BufRead;

fn main() {
    let filename = "calories.txt";
    let file = File::open(filename).unwrap();
    let reader  = std::io::BufReader::new(file);
    let mut max_calories: i32 = 0;
    let mut current_calories: i32 = 0;

    for  line in reader.lines() {
        let line = line.unwrap();
        if line != "" {
            let calories: i32 = line.parse().unwrap();
            current_calories += calories;
        } else {
            if current_calories > max_calories {
                max_calories = current_calories;
            }
            current_calories = 0;
        }
    }
    print!("{}", max_calories);
}
