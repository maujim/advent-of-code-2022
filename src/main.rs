use clap::Parser;
use itertools::Itertools;
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

    // println!("{}", day1::solve(&mut buf_reader)?);
    // println!("{}", day2::solve(&mut buf_reader)?);
    // println!("{}", day3::solve(&mut buf_reader)?);
    println!("{}", day4::solve(&mut buf_reader)?);

    Ok(())
}

mod day4 {
    use super::*;
    use std::io::SeekFrom;

    use std::collections::{HashMap, HashSet};

    fn line_to_digit(line: String) -> [u32; 4] {
        let (first, second) = line.split_once(",").unwrap();

        let first: Vec<_> = first
            .split("_")
            .into_iter()
            .take(2)
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .collect();

        let second: Vec<_> = second
            .split("_")
            .into_iter()
            .take(2)
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .collect();

        [first[0], first[1], second[0], second[1]]
    }

    pub fn solve(reader: &mut BufReader<File>) -> Result<Answer, Box<dyn error::Error>> {
        let char2score: HashMap<char, u32> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i as u32 + 1))
            .collect();

        reader
            .lines()
            .map(Result::unwrap)
            .map(|encoded| {
                let half_length = encoded.len() / 2;

                let first_rucksack: HashSet<char> = encoded.chars().take(half_length).collect();
                let second_rucksack = encoded.chars().skip(half_length);

                second_rucksack
                    .filter(|elem| first_rucksack.contains(elem))
                    .collect::<Vec<_>>()
            })
            .for_each(|s| println!("{s:?}"));

        let part1 = 5 as u32;

        reader.seek(SeekFrom::Start(0))?;

        Ok(Answer {
            part1: part1.to_string(),
            part2: part1.to_string(),
        })
    }
}
mod day3 {
    use super::*;
    use std::io::SeekFrom;

    use std::collections::{HashMap, HashSet};

    pub fn solve(reader: &mut BufReader<File>) -> Result<Answer, Box<dyn error::Error>> {
        let char2score: HashMap<char, u32> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i as u32 + 1))
            .collect();

        let part1: u32 = reader
            .lines()
            .map(Result::unwrap)
            .flat_map(|ref encoded| {
                let half_length = encoded.len() / 2;

                let first_rucksack: HashSet<char> = encoded.chars().take(half_length).collect();
                let second_rucksack = encoded.chars().skip(half_length);

                second_rucksack
                    .filter(|elem| first_rucksack.contains(elem))
                    .collect::<Vec<_>>()
            })
            .map(|ref s| char2score.get(s).unwrap())
            .sum();

        reader.seek(SeekFrom::Start(0))?;

        let part2: u32 = reader
            .lines()
            .map(Result::unwrap)
            .chunks(3)
            .into_iter()
            .map(|group| {
                let group: Vec<_> = group.into_iter().collect();
                let hash1: HashSet<char> = group[1].chars().collect();
                let hash2: HashSet<char> = group[2].chars().collect();

                group[0]
                    .chars()
                    .find(|elem| hash1.contains(elem) && hash2.contains(elem))
                    .expect("there is always one common letter per group")
            })
            .map(|ref s| char2score.get(s).unwrap())
            .sum();

        Ok(Answer {
            part1: part1.to_string(),
            part2: part2.to_string(),
        })
    }
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
                Outcome::Draw => *self,
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
                let opponent = &RPS::new(chars.next().unwrap());
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
            if line.is_empty() {
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
