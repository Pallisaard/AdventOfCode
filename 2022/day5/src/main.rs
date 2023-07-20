use std::{fs::File, io::BufRead, io::BufReader};

mod parser;

fn make_single_move(stacks: &mut Vec<Vec<char>>, mv: parser::Move) -> Result<(), String> {
    for _ in 0..mv.num {
        let popped = stacks[mv.from_index()]
            .pop()
            .ok_or("make_single_move: pop failed".to_string())?;
        stacks[mv.to_index()].push(popped);
    }
    Ok(())
}

fn make_advanced_move(stacks: &mut Vec<Vec<char>>, mv: parser::Move) -> Result<(), String> {
    let s_len = stacks[mv.from_index()].len();
    println!("s_len: {}", s_len);
    println!("mv.num: {}", mv.num);
    let split_pos = s_len - mv.num as usize;
    let mut popped = stacks[mv.from_index()].split_off(split_pos);
    stacks[mv.to_index()].append(&mut popped);

    Ok(())
}

fn grab_top(stacks: Vec<Vec<char>>) -> Vec<char> {
    let mut top = Vec::new();
    for stack in stacks {
        if let Some(c) = stack.last() {
            top.push(*c);
        }
    }
    top
}

fn main() -> Result<(), String> {
    let fname = "input.txt";
    let f = File::open(fname).expect("Unable to open file");
    let reader = BufReader::new(f);
    let mut lines = reader.lines().map(|l| l.unwrap());

    let mut stack_lines: Vec<String> = Vec::new();
    let mut move_lines: Vec<String> = Vec::new();
    let mut stack_done = false;

    while let Some(line) = lines.next() {
        if line == "" {
            stack_done = true;
            continue;
        }
        if !stack_done {
            stack_lines.push(line);
        } else {
            move_lines.push(line);
        }
    }

    let mut stacks = parser::parse_stacks(&mut stack_lines)?;

    let moves = parser::parse_moves(&mut move_lines)?;

    println!("stacks:\n{:?}", stacks);
    println!("moves:\n{:?}", moves);

    for mv in moves {
        make_single_move(&mut stacks, mv.clone())?;
        println!("new stack after move:\n{:?}\n{:?}", mv, stacks);
    }

    println!("new stacks:\n{:?}", stacks);

    let top = grab_top(stacks);
    let top_string = top.iter().collect::<String>();

    println!("top string: {}", top_string);

    println!("");
    println!("part 2.");
    println!("");

    let f2 = File::open(fname).expect("Unable to open file");
    let reader2 = BufReader::new(f2);
    let mut lines2 = reader2.lines().map(|l| l.unwrap());

    let mut stack_lines2: Vec<String> = Vec::new();
    let mut move_lines2: Vec<String> = Vec::new();
    let mut stack_done2 = false;

    while let Some(line) = lines2.next() {
        if line == "" {
            stack_done2 = true;
            continue;
        }
        if !stack_done2 {
            stack_lines2.push(line);
        } else {
            move_lines2.push(line);
        }
    }

    let mut stacks2 = parser::parse_stacks(&mut stack_lines2)?;

    let moves2 = parser::parse_moves(&mut move_lines2)?;

    println!("stacks2:\n{:?}", stacks2);

    for mv in moves2 {
        make_advanced_move(&mut stacks2, mv.clone())?;
        println!("new stack after move:\n{:?}\n{:?}", mv, stacks2);
    }

    println!("new stacks:\n{:?}", stacks2);

    let top2 = grab_top(stacks2);
    let top_string2 = top2.iter().collect::<String>();

    println!("top string2: {}", top_string2);

    Ok(())
}
