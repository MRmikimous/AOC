use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::process::exit;
use std::ptr::null;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <filename>", &args[0]);
        exit(0)
    }

    let path = &args[1];
    let mut file = match File::open(path) {
        Err(e) => panic!("couldn't open {}: {}", path, e),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let total = get_total(reader);


    println!("{}", total)
}

fn get_total(reader: BufReader<File>) -> u32 {
    let mut total= 0;
    for line in reader.lines() {
        let mut line = match line {
            Err(e) => panic!("failed reading line from file"),
            Ok(line) => line,
        };

        let mut digit = "".to_string();

        for char in line.chars() {
            if char.is_digit(10) {
                digit.push(char);
                break
            }

        };

        if digit.len() == 0 {
            break
        }

        for char in line.chars().rev() {
            if char.is_digit(10) {
                digit.push(char);
                break
            }

        };

        total += match digit.parse::<u32>() {
            Err(e) => panic!("failed to parse String to u32"),
            Ok(line) => line,
        }
    }

    total
}