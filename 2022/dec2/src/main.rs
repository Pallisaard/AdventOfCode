use std::{fs::File, io::BufRead, io::BufReader};

mod rps;

fn main() {
    let fname = "input.txt";
    let f = File::open(fname).expect("Unable to open file");
    let fstream = BufReader::new(f);

    let games = fstream.lines().map(|line| line.unwrap());

    let total_score = games.map(|game| rps::game_to_score(&game)).sum::<i32>();

    println!("Total score: {}", total_score);

    let f2 = File::open(fname).expect("Unable to open file");
    let fstream_2 = BufReader::new(f2);

    let games_2 = fstream_2.lines().map(|line| line.unwrap());

    let total_score_2 = games_2.map(|game| rps::game_to_score_2(&game)).sum::<i32>();

    println!("Total score 2: {:?}", total_score_2);
}
