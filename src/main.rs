use std::include_str;

#[derive(Debug, PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scisors,
}
impl From<&str> for RPS {
    fn from(input: &str) -> RPS {
        match input {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scisors,
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
    pub fn win_loss_score(&self, hand_2: &RPS) -> u32 {
        match (self, hand_2) {
            (&RPS::Rock, &RPS::Paper) => 0,
            (&RPS::Paper, &RPS::Scisors) => 0,
            (&RPS::Scisors, &RPS::Rock) => 0,
            (&RPS::Paper, &RPS::Rock) => 6,
            (&RPS::Scisors, &RPS::Paper) => 6,
            (&RPS::Rock, &RPS::Scisors) => 6,
            _ => 3,
        }
    }
    pub fn score_hands(hand_1: &RPS, hand_2: &RPS) -> (u32, u32) {
        let score_1 = hand_1.rps_score() + hand_1.win_loss_score(hand_2);
        let score_2 = hand_2.rps_score() + hand_2.win_loss_score(hand_1);
        (score_1, score_2)
    }
}

// top 3
pub fn main() {
    let vec = include_str!("input.txt")
        .lines()
        .map(|line| line.split(" ").map(|char| RPS::from(char)).collect::Vec<_>());

    // vec.sort_by(|a, b| b.cmp(a));

    // let temp: u32 = vec.iter().take(3).sum();

    println!("{:?}", vec);
}
