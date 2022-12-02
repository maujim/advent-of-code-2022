use clap::Parser;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Parser)]
struct Cli {
    input: String,
}

#[derive(Debug)]
pub struct Answer {
    part1: String,
    part2: String,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "part1: {} part2: {}", self.part1, self.part2)
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();
    let mut buf_reader = BufReader::new(File::open(cli.input)?);

    // let answer = day1::solve(&mut buf_reader)?;
    let answer = day2::solve(&mut buf_reader)?;

    println!("{answer}");

    Ok(())
}

mod day2 {
    use super::*;
    use std::io::SeekFrom;
    use std::mem;

    #[derive(Copy, Clone)]
    enum RPS {
        Rock,
        Paper,
        Scissors,
    }
    enum Outcome {
        Win,
        Loss,
        Draw,
    }

    impl RPS {
        fn new(c: char) -> Self {
            match c {
                'A' | 'X' => Self::Rock,
                'B' | 'Y' => Self::Paper,
                'C' | 'Z' => Self::Scissors,
                _ => unreachable!(),
            }
        }

        fn compare(&self, other: &Self) -> Outcome {
            if mem::discriminant(self) == mem::discriminant(other) {
                return Outcome::Draw;
            }

            match (self, other) {
                (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper) => Outcome::Win,
                _ => Outcome::Loss,
            }
        }

        // what comes after this one in the chain
        fn after(&self) -> Self {
            match self {
                Self::Rock => Self::Scissors,
                Self::Scissors => Self::Paper,
                Self::Paper => Self::Rock,
            }
        }

        fn find_complement(&self, outcome: &Outcome) -> Self {
            match outcome {
                Outcome::Win => self.after().after(),
                Outcome::Loss => self.after(),
                Outcome::Draw => self.clone(),
            }
        }
    }

    impl Outcome {
        fn new(c: char) -> Self {
            match c {
                'X' => Self::Loss,
                'Y' => Self::Draw,
                'Z' => Self::Win,
                _ => unreachable!(),
            }
        }
    }

    pub fn solve(reader: &mut BufReader<File>) -> Result<Answer, Box<dyn error::Error>> {
        let part1: i64 = reader
            .lines()
            .map(Result::unwrap)
            .map(|line| {
                let mut chars = line.chars();
                let ref opponent = RPS::new(chars.next().unwrap());
                chars.next();
                let selected = RPS::new(chars.next().unwrap());

                let points_from_selection = match selected {
                    RPS::Rock => 1,
                    RPS::Paper => 2,
                    RPS::Scissors => 3,
                };

                let points_from_outcome = match selected.compare(opponent) {
                    Outcome::Win => 6,
                    Outcome::Loss => 0,
                    Outcome::Draw => 3,
                };
                points_from_selection + points_from_outcome
            })
            .sum();

        reader.seek(SeekFrom::Start(0))?;

        let part2: i64 = reader
            .lines()
            .map(Result::unwrap)
            .map(|line| {
                let mut chars = line.chars();
                let opponent = RPS::new(chars.next().unwrap());
                chars.next();
                let outcome = Outcome::new(chars.next().unwrap());

                let selected = opponent.find_complement(&outcome);

                let points_from_selection = match selected {
                    RPS::Rock => 1,
                    RPS::Paper => 2,
                    RPS::Scissors => 3,
                };

                let points_from_outcome = match outcome {
                    Outcome::Win => 6,
                    Outcome::Loss => 0,
                    Outcome::Draw => 3,
                };

                points_from_selection + points_from_outcome
            })
            .sum();

        Ok(Answer {
            part1: part1.to_string(),
            part2: part2.to_string(),
        })
    }
}

mod day1 {
    use super::*;

    fn insert_and_sort(data: &mut [i64; 3], x: i64) {
        if x > data[0] {
            data[0] = x;
            data.sort();
        }
    }

    pub fn solve(reader: &mut BufReader<File>) -> Result<Answer, Box<dyn error::Error>> {
        let mut sum = 0;
        let mut sorted_calorie_counts = [0, 0, 0];

        for line in reader.lines().map(Result::unwrap) {
            if line == "" {
                insert_and_sort(&mut sorted_calorie_counts, sum);
                sum = 0;
            } else {
                sum += line.parse::<i64>()?;
            }
        }

        Ok(Answer {
            part1: sorted_calorie_counts[2].to_string(),
            part2: sorted_calorie_counts.iter().sum::<i64>().to_string(),
        })
    }
}
