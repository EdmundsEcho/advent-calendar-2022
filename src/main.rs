use std::include_str;
use std::iter::zip;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSE: u32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub enum RPS {
    Rock,
    Paper,
    Scisors,
}
#[derive(Debug, PartialEq, Eq)]
pub enum WLD {
    Win,
    Lose,
    Draw,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hands {
    first_hand: RPS,
    goal: WLD,
}
impl Hands {
    fn get_score(&self) -> u32 {
        match self {
            Hands { goal: WLD::Win, .. } => self.win(),
            Hands {
                goal: WLD::Lose, ..
            } => self.lose(),
            Hands {
                goal: WLD::Draw, ..
            } => self.draw(),
        }
    }
    fn win(&self) -> u32 {
        let score = match self.first_hand {
            RPS::Rock => RPS::Paper.rps_score(),
            RPS::Paper => RPS::Scisors.rps_score(),
            RPS::Scisors => RPS::Rock.rps_score(),
        };
        score + WIN
    }
    fn lose(&self) -> u32 {
        let score = match self.first_hand {
            RPS::Rock => RPS::Scisors.rps_score(),
            RPS::Paper => RPS::Rock.rps_score(),
            RPS::Scisors => RPS::Paper.rps_score(),
        };
        score + LOSE
    }
    fn draw(&self) -> u32 {
        let score = match self.first_hand {
            RPS::Rock => RPS::Rock.rps_score(),
            RPS::Paper => RPS::Paper.rps_score(),
            RPS::Scisors => RPS::Scisors.rps_score(),
        };
        score + DRAW
    }
}
impl From<&str> for Hands {
    fn from(input: &str) -> Hands {
        let pair: Vec<&str> = input.split(" ").collect();
        Hands {
            first_hand: pair[0].into(),
            goal: pair[1].into(),
        }
    }
}
// The strategy
// X -> must lose
// Y -> must draw
// Z -> must win (based on what the other has)
impl From<&str> for WLD {
    fn from(input: &str) -> WLD {
        match input {
            "X" => WLD::Lose,
            "Y" => WLD::Draw,
            "Z" => WLD::Win,
            _ => panic!("Invalid input: {input}"),
        }
    }
}
impl From<&str> for RPS {
    fn from(input: &str) -> RPS {
        match input {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scisors,
            _ => panic!("Invalid input: {input}"),
        }
    }
}
impl RPS {
    fn rps_score(&self) -> u32 {
        match self {
            &RPS::Rock => 1,
            &RPS::Paper => 2,
            &RPS::Scisors => 3,
        }
    }
    fn win_loss_score(&self, hand_2: &RPS) -> u32 {
        match (self, hand_2) {
            (&RPS::Rock, &RPS::Paper) => LOSE,
            (&RPS::Paper, &RPS::Scisors) => LOSE,
            (&RPS::Scisors, &RPS::Rock) => LOSE,
            (&RPS::Paper, &RPS::Rock) => WIN,
            (&RPS::Scisors, &RPS::Paper) => WIN,
            (&RPS::Rock, &RPS::Scisors) => WIN,
            _ => DRAW,
        }
    }
}

pub fn score_hands(hand_1: &RPS, hand_2: &RPS) -> (u32, u32) {
    let score_1 = hand_1.rps_score() + hand_1.win_loss_score(hand_2);
    let score_2 = hand_2.rps_score() + hand_2.win_loss_score(hand_1);
    (score_1, score_2)
}

pub fn main() {
    let vec = include_str!("input.txt")
        .lines()
        .map(|line| {
            // here how read in RPS WLD... maybe nom parser
            Hands::from(line)
        })
        .collect::<Vec<Hands>>();

    let result = vec
        .iter()
        .map(|hands| hands.get_score())
        .collect::<Vec<u32>>();

    let zipped = zip(&vec, &result).collect::<Vec<(&Hands, &u32)>>();

    println!("{:?}", vec);
    println!("{:?}", result);
    println!("{:?}", zipped);
    println!("Sum: {:?}", result.iter().sum::<u32>());
}
