fn parse_line(line: String) -> Result<(u32, u32, u32, u32), String> {
    let mut chars = line.chars();
    let mut buf = String::new();
    let mut vals: Vec<u32> = Vec::new();

    while let Some(c) = chars.next() {
        if c == '-' || c == ',' {
            let val = buf.parse::<u32>().map_err(|e| e.to_string())?;
            vals.push(val);
            buf.clear();
        } else if c.is_digit(10) {
            buf.push(c);
        } else {
            return Err(format!("Invalid character: {}", c));
        }
    }
    let val = buf.parse::<u32>().map_err(|e| e.to_string())?;
    vals.push(val);

    if vals.len() != 4 {
        return Err("Invalid number of values".to_string());
    }
    Ok((vals[0], vals[1], vals[2], vals[3]))
}

fn are_ranges_fully_overlapping(range1: (u32, u32), range2: (u32, u32)) -> bool {
    let r2_in_r1 = range1.0 >= range2.0 && range1.1 <= range2.1;
    let r1_in_r2 = range1.0 <= range2.0 && range1.1 >= range2.1;
    r1_in_r2 || r2_in_r1
}

fn are_ranges_partially_overlapping(range1: (u32, u32), range2: (u32, u32)) -> bool {
    let r2_in_r1 = range1.0 <= range2.0 && range1.1 >= range2.0;
    let r1_in_r2 = range1.0 <= range2.1 && range1.1 >= range2.1;
    r1_in_r2 || r2_in_r1 || are_ranges_fully_overlapping(range1, range2)
}

pub fn check_line_ranges(line: String) -> Result<bool, String> {
    let (a, b, c, d) = parse_line(line)?;
    let range1 = (a, b);
    let range2 = (c, d);
    Ok(are_ranges_fully_overlapping(range1, range2))
}

pub fn check_line_ranges_partially(line: String) -> Result<bool, String> {
    let (a, b, c, d) = parse_line(line)?;
    let range1 = (a, b);
    let range2 = (c, d);
    Ok(are_ranges_partially_overlapping(range1, range2))
}
