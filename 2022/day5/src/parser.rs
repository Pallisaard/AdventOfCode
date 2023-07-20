#[derive(Debug, Clone)]
pub struct Move {
    pub num: i32,
    pub from: usize,
    pub to: usize,
}

impl Move {
    pub fn from_index(&self) -> usize {
        self.from - 1
    }

    pub fn to_index(&self) -> usize {
        self.to - 1
    }
}

pub fn parse_any_char(buffer: &mut impl Iterator<Item = char>) -> Result<char, String> {
    buffer.next().ok_or("could not parse character".to_string())
}

pub fn parse_char(buffer: &mut impl Iterator<Item = char>, c: char) -> Result<char, String> {
    let out_char = parse_any_char(buffer)?;
    match out_char == c {
        true => Ok(out_char),
        false => Err(format!("expected \"{}\"", c)),
    }
}

pub fn parse_integer(buffer: &mut impl Iterator<Item = char>) -> Result<i32, String> {
    let mut out_int = String::new();
    while let Some(c) = parse_any_char(buffer).ok() {
        if c.is_digit(10) {
            out_int.push(c);
        } else {
            break;
        }
    }
    Ok(out_int.parse::<i32>().map_err(|e| e.to_string())?)
}

pub fn parse_string(buffer: &mut impl Iterator<Item = char>, s: &str) -> Result<String, String> {
    let mut out_string = String::new();
    for c in s.chars() {
        out_string.push(parse_char(buffer, c)?);
    }
    Ok(out_string)
}

pub fn parse_symbol(buffer: &mut impl Iterator<Item = char>, s: &str) -> Result<String, String> {
    let out = parse_string(buffer, s)?;
    parse_whitespace(buffer)?;
    Ok(out)
}

pub fn parse_whitespace(buffer: &mut impl Iterator<Item = char>) -> Result<(), String> {
    let whitespace = parse_any_char(buffer)?;
    match whitespace.is_whitespace() {
        true => Ok(()),
        false => Err("expected whitespace".to_string()),
    }
}

pub fn parse_crate_letters(
    buffer: &mut impl Iterator<Item = char>,
) -> Result<(char, char, char), String> {
    let sqr_brac_begin = parse_any_char(buffer)?;
    let out_char = parse_any_char(buffer)?;
    let sqr_brac_end = parse_any_char(buffer)?;
    Ok((sqr_brac_begin, out_char, sqr_brac_end))
}

pub fn parse_crate_letter_or_none(
    buffer: &mut impl Iterator<Item = char>,
) -> Result<Option<char>, String> {
    let (sqr_begin, out, sqr_end) = parse_crate_letters(buffer)?;
    match (sqr_begin, out, sqr_end) {
        ('[', out, ']') => Ok(Some(out)),
        (' ', ' ', ' ') => Ok(None),
        _ => Err("expected crate to be \"[*char*]\" or \"   \"".to_string()),
    }
}

pub fn parse_crate_line(
    buffer: &mut impl Iterator<Item = char>,
    n_stacks: usize,
) -> Result<Vec<Option<char>>, String> {
    let mut crate_line: Vec<Option<char>> = Vec::new();
    for _ in 0..(n_stacks - 1) {
        crate_line.push(parse_crate_letter_or_none(buffer)?);
        parse_whitespace(buffer)?; // space
    }
    crate_line.push(parse_crate_letter_or_none(buffer)?);
    Ok(crate_line)
}

fn build_stack_structure(stacks: Vec<Vec<Option<char>>>) -> Result<Vec<Vec<char>>, String> {
    let mut stack_structure: Vec<Vec<char>> = vec![Vec::new(); stacks[0].len()];
    for stack in stacks.iter().rev() {
        for (i, crt) in stack.iter().enumerate() {
            match crt {
                Some(c) => stack_structure[i].push(*c),
                None => (),
            }
        }
    }
    Ok(stack_structure)
}

pub fn parse_stacks(stack_strings: &mut Vec<String>) -> Result<Vec<Vec<char>>, String> {
    let mut stacks: Vec<Vec<Option<char>>> = Vec::new();

    let stack_size = stack_strings
        .pop()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .len();

    for stack_string in stack_strings {
        let mut buffer = stack_string.chars().peekable();
        stacks.push(parse_crate_line(&mut buffer, stack_size)?);
    }

    build_stack_structure(stacks)
}

pub fn parse_move_line(buffer: &mut impl Iterator<Item = char>) -> Result<Move, String> {
    parse_symbol(buffer, "move")?;
    // let n_moves = parse_any_char(buffer)?
    //     .to_digit(10)
    //     .ok_or("n_moves: Expected digit".to_string())? as i32;
    // parse_whitespace(buffer)?;

    let n_moves = parse_integer(buffer)?;

    parse_symbol(buffer, "from")?;
    // let start_stack = parse_any_char(buffer)?
    //     .to_digit(10)
    //     .ok_or("start_stack: Expected digit".to_string())? as usize;
    // parse_whitespace(buffer)?;

    let start_stack = parse_integer(buffer)? as usize;

    parse_symbol(buffer, "to")?;
    let end_stack = parse_any_char(buffer)?
        .to_digit(10)
        .ok_or("end_stack: Expected digit".to_string())? as usize;

    let mv = Move {
        num: n_moves,
        from: start_stack,
        to: end_stack,
    };

    Ok(mv)
}

pub fn parse_moves(move_lines: &mut Vec<String>) -> Result<Vec<Move>, String> {
    let mut moves: Vec<Move> = Vec::new();

    for move_line in move_lines {
        let mut buffer = move_line.chars();
        moves.push(parse_move_line(&mut buffer)?);
    }

    Ok(moves)
}
