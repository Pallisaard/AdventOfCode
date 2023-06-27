use std::{fs::File, io::BufRead, io::BufReader};

mod dec4;

fn main() {
    let fname = "input.txt";
    let f = File::open(fname).expect("Unable to open file");
    let fstream = BufReader::new(f);
    let lines = fstream.lines().map(|l| l.unwrap());

    let n_ranges_overlapping = lines
        .map(|l| dec4::check_line_ranges(l))
        .filter(|r| r.is_ok() && r.clone().unwrap())
        .count();
    // .collect::<Result<Vec<bool>, String>>();

    println!("Number of overlapping ranges: {:?}", n_ranges_overlapping);

    let f2 = File::open(fname).expect("Unable to open file");
    let fstream2 = BufReader::new(f2);
    let lines2 = fstream2.lines().map(|l| l.unwrap());

    let n_ranges_partially_overlapping = lines2
        .map(|l| dec4::check_line_ranges_partially(l))
        .filter(|r| r.is_ok() && r.clone().unwrap())
        .count();
    // .collect::<Result<Vec<bool>, String>>();

    println!(
        "Number of partially overlapping ranges: {:?}",
        n_ranges_partially_overlapping
    );
}
