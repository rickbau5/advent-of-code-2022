use crate::input;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    fn points(self) -> i32 {
        return match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }; 

    }

    fn outcome(self, other: Move) -> Outcome {
        if self == other {
            return Outcome::Draw;
        }
        
        if (self == Move::Rock && other == Move::Scissors) ||
            (self == Move::Paper && other == Move::Rock) ||
            (self == Move::Scissors && other == Move::Paper) {
            return Outcome:: Win;
        }

        return Outcome::Lose;
    }
}


enum Outcome {
    Lose,
    Draw,
    Win
}

impl Outcome {
    fn points(&self) -> i32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}


pub fn run() -> (String, String) {
    let input = input::load_input(2);

    let mut total_p1 = 0;
    let mut total_p2 = 0;
    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let opponent = parts.get(0).unwrap();
        let me = parts.get(1).unwrap();

        let opponent_move = match *opponent {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _=>panic!("unknown value {}", opponent)
        };

        let my_move = match *me {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _=> panic!("unknown value {}", opponent)
        };

        let score_p1 = my_move.points() + my_move.outcome(opponent_move).points();

        // part 2
        let my_move_p2 = match *me {
            // lose
            "X" => match opponent_move {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            // draw
            "Y" => opponent_move,
            // win
            "Z" => match opponent_move {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            _=> panic!("unknown value {}", opponent)
        };

        let score_p2 = my_move_p2.points() + my_move_p2.outcome(opponent_move).points();

        total_p1 += score_p1;
        total_p2 += score_p2;
    }

    return (total_p1.to_string(), total_p2.to_string())
}
