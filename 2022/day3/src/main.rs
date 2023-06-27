use std::collections::HashSet;
use std::{fs::File, io::BufRead, io::BufReader};

// * each line has a bunch of alphabet characters, lowercase and uppercase, and with an even number of characters.
// * Each letter is valued at its position in the alphabet, starting at 1 for lowercase and 27 for uppercase.
// * each line is split into 2 parts of equal size.
// * For each line, find the letters that appear in both parts and sum up their values.
// * Compute the total sum by summing results from each line.

fn split_line(line: String) -> (String, String) {
    let len = line.len();
    let mid = len / 2;
    let (first, second) = line.split_at(mid);
    (first.to_string(), second.to_string())
}

fn get_letters_shared_by_strings(first: String, second: String) -> HashSet<char> {
    let mut letters: HashSet<char> = HashSet::new();
    for c in first.chars() {
        if second.contains(c) {
            letters.insert(c);
        }
    }
    letters
}

fn get_letter_value(c: char) -> u32 {
    let mut val = c as u32;
    if val >= 97 {
        val -= 96;
    } else {
        val -= 64 - 26;
    }
    val
}

fn get_sum_of_letter_values(letters: HashSet<char>) -> u32 {
    let mut sum = 0;
    for c in letters {
        sum += get_letter_value(c);
    }
    sum
}

fn get_priority_sum_from_line(line: String) -> u32 {
    let (first, second) = split_line(line);
    let letters = get_letters_shared_by_strings(first, second);
    get_sum_of_letter_values(letters)
}

fn find_common_letter(first: String, second: String, third: String) -> Option<char> {
    for c in first.chars() {
        if second.contains(c) && third.contains(c) {
            return Some(c);
        }
    }
    None
}

fn get_badge_sum_from_line(first: String, second: String, third: String) -> Option<u32> {
    let common_letter = find_common_letter(first, second, third);
    match common_letter {
        Some(c) => Some(get_letter_value(c)),
        None => None,
    }
}

fn main() {
    let fname = "input_test.txt";
    let f = File::open(fname).expect("Unable to open file");
    let fstream = BufReader::new(f);
    let lines = fstream.lines().map(|line| line.unwrap());

    let total_priority_of_dublicate_letters = lines
        .map(|line| get_priority_sum_from_line(line))
        .sum::<u32>();

    println!(
        "Total priority of dublicate letters: {:?}",
        total_priority_of_dublicate_letters
    );

    let f2 = File::open("input.txt").expect("Unable to open file");
    let fstream2 = BufReader::new(f2);
    let mut lines2 = fstream2.lines().map(|line| line.unwrap());

    let mut total_priority_of_badges: u32 = 0;

    while let Some(first) = lines2.next() {
        let second = lines2.next().unwrap();
        let third = lines2.next().unwrap();
        total_priority_of_badges += get_badge_sum_from_line(first, second, third).unwrap();
    }

    println!("Total priority of badges: {:?}", total_priority_of_badges);

    println!("Hello, world!");
}
