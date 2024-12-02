use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let mut to_count = Vec::<i32>::new();
    let mut counter = HashMap::<i32, i32>::new();

    let input_file_path = "./input";
    if let Ok(file) = read_to_string(input_file_path) {
        file.lines().for_each(|line| {
            let mut split = line.split("   ");
            let n1 = split.next().expect("malformed input??");
            let n2 = split.next().expect("malformed input??").parse::<i32>().expect("malformed number");
            if !counter.contains_key(&n2) {
                counter.insert(n2, 1);
            } else {
                counter.insert(n2, counter.get(&n2).expect("bruh")+1);
            }

            to_count.push(n1.parse::<i32>().expect("malformed number??"));
        });
    } else {
        eprintln!("Failed to read file");
    }

    let rez = to_count.iter().fold(0, |acc, x| {
        acc + if let Some(n) = counter.get(x) { x * n } else { 0 }
    });
    println!("{rez}");
}
