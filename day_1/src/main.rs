use std::fs::read_to_string;
use std::iter::zip;

fn main() {
    let mut list_left = Vec::<i32>::new();
    let mut list_right = Vec::<i32>::new();

    let input_file_path = "./input";
    if let Ok(file) = read_to_string(input_file_path) {
        file.lines().for_each(|line| {
            let mut split = line.split("   ");
            let n1 = split.next().expect("malformed input??");
            let n2 = split.next().expect("malformed input??");

            list_left.push(n1.parse::<i32>().expect("malformed number??"));
            list_right.push(n2.parse::<i32>().expect("malformed number??"));
        });
    } else {
        eprintln!("Failed to read file");
    }

    list_left.sort();
    list_right.sort();

    let rez = zip(list_left, list_right).fold(0, |acc, x| acc + (x.0-x.1).abs());
    println!("{rez}");
}
