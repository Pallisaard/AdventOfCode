enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn play_game(opponent_str: &Shape, player_str: &Shape) -> Outcome {
    match (opponent_str, player_str) {
        (Shape::Rock, Shape::Rock) => Outcome::Draw,
        (Shape::Rock, Shape::Paper) => Outcome::Loss,
        (Shape::Rock, Shape::Scissors) => Outcome::Win,
        (Shape::Paper, Shape::Rock) => Outcome::Win,
        (Shape::Paper, Shape::Paper) => Outcome::Draw,
        (Shape::Paper, Shape::Scissors) => Outcome::Loss,
        (Shape::Scissors, Shape::Rock) => Outcome::Loss,
        (Shape::Scissors, Shape::Paper) => Outcome::Win,
        (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
    }
}

fn shape_to_score(shape: Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}

fn outcome_to_score(outcome: Outcome) -> i32 {
    match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn parse_game(game: &str) -> (Shape, Outcome) {
    let mut parts = game.split_whitespace();
    let opponent_str = parts.next();
    let player_str = parts.next();

    let opponent_shape = match opponent_str {
        Some("A") => Shape::Rock,
        Some("B") => Shape::Paper,
        Some("C") => Shape::Scissors,
        _ => panic!("Bad opponent shape"),
    };

    let player_shape = match player_str {
        Some("X") => Shape::Rock,
        Some("Y") => Shape::Paper,
        Some("Z") => Shape::Scissors,
        _ => panic!("Bad player shape"),
    };

    let outcome = play_game(&player_shape, &opponent_shape);

    (player_shape, outcome)
}

pub fn game_to_score(game: &str) -> i32 {
    let (shape, outcome) = parse_game(game);
    shape_to_score(shape) + outcome_to_score(outcome)
}

fn shape_choice_to_make_outcome(opponent_shape: &Shape, outcome: &Outcome) -> Shape {
    match outcome {
        Outcome::Loss => match opponent_shape {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
        Outcome::Draw => match opponent_shape {
            Shape::Rock => Shape::Rock,
            Shape::Paper => Shape::Paper,
            Shape::Scissors => Shape::Scissors,
        },
        Outcome::Win => match opponent_shape {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
    }
}

fn parse_game_2(game: &str) -> (Shape, Outcome) {
    let mut parts = game.split_whitespace();
    let opponent_str = parts.next();
    let outcome_str = parts.next();

    let opponent_shape = match opponent_str {
        Some("A") => Shape::Rock,
        Some("B") => Shape::Paper,
        Some("C") => Shape::Scissors,
        _ => panic!("Bad opponent shape"),
    };

    let outcome = match outcome_str {
        Some("X") => Outcome::Loss,
        Some("Y") => Outcome::Draw,
        Some("Z") => Outcome::Win,
        _ => panic!("Bad player shape"),
    };

    let player_shape = shape_choice_to_make_outcome(&opponent_shape, &outcome);

    (player_shape, outcome)
}

pub fn game_to_score_2(game: &str) -> i32 {
    let (shape, opponent_outcome) = parse_game_2(game);
    shape_to_score(shape) + outcome_to_score(opponent_outcome)
}
