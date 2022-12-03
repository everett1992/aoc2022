use aoc2022::SumCalories;
use clap::{Parser, Subcommand};
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, subcommand_required(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    CalorieCounting {
        input: PathBuf,
        /// find top n elves
        #[arg(short, long, default_value_t = 1)]
        top: u8,
    },

    RockPaperScissors {
        input: PathBuf,
    },
}

fn main() -> Result<(), CliError> {
    env_logger::init();
    let args = Cli::parse();
    match args.command {
        Some(Commands::CalorieCounting { input, top }) => {
            let input = lines(input)?;
            println!("{}", calorie_counting(input, top)?);
        },
        Some(Commands::RockPaperScissors { input }) => {
            let input = lines(input)?;
            let (part_1, part_2) = rock_paper_scissors(input)?;
            println!("part 2: {}", part_1);
            println!("part 1: {}", part_2);
        },
        _ => panic!(),
    };
    Ok(())
}

// Points
//  W = 6 L = 0 T = 3
//  R = 1 P = 2 S = 3

fn rock_paper_scissors<I>(input: I) -> Result<(u32, u32), CliError>
where I: IntoIterator<Item = String> {
    let (part_1, part_2) = input.into_iter().fold(
        (0u32, 0u32),
        |(part_1, part_2), line| {
            (
                part_1 + score_rps_game_part_1(line.as_str()),
                part_2 + score_rps_game_part_2(line.as_str()),
            )
        },
    );
    Ok((part_1, part_2))
}

fn score_rps_game_part_1(line: &str) -> u32 {
    // Them
    //  A Rock
    //  B Paper
    //  C Scissors
    //
    // Me
    //  X Rock
    //  Y Paper
    //  Z Scissors
    match line {
        //       Win Threw
        "A X" => 3 + 1,
        "A Y" => 6 + 2,
        "A Z" => 0 + 3,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 6 + 1,
        "C Y" => 0 + 2,
        "C Z" => 3 + 3,
        _ => 0,
    }
}

fn score_rps_game_part_2(line: &str) -> u32 {
    // Them
    //  A Rock
    //  B Paper
    //  C Scissors
    //
    // Me
    //  X Lose
    //  Y Draw
    //  Z Win
    match line {
        //       Win Threw
        "A X" => 0 + 3,
        "A Y" => 3 + 1,
        "A Z" => 6 + 2,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 0 + 2,
        "C Y" => 3 + 3,
        "C Z" => 6 + 1,
        _ => 0,
    }
}

fn lines(input: PathBuf) -> Result<impl Iterator<Item = String>, CliError> {
    let file = File::open(input)?;
    let iter = BufReader::new(file).lines()
        .map(|e| e.unwrap());
    return Ok(iter);
}

fn calorie_counting<I>(input: I, n: u8) -> Result<u32, CliError>
where I: IntoIterator<Item = String> {
    let elves = input.into_iter().sum_calories();

    let mut top_n = vec![0u32; n as usize];
    for elf in elves {
        let elf = elf?;
        if elf > top_n[0] {
            top_n[0] = elf;
            top_n.sort();
        }
    }
    let sum = top_n.iter().sum();
    return Ok(sum);
}

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

#[cfg(test)]
mod tests {
    use crate::{calorie_counting, rock_paper_scissors};

    #[test]
    fn sums_top_n_elves_calories() {
        let iter = vec![
            "1", // elf 1
            "",
            "2", // elf 2
            "",
            "1", // elf 3
            "2",
        ].into_iter().map(String::from);

        assert_eq!(calorie_counting(iter, 2).unwrap(), 5);
    }

    #[test]
    fn scores_rps() {
        let iter = vec![
            "A Y",
            "B X",
            "C Z",
        ].into_iter().map(String::from);

        assert_eq!(rock_paper_scissors(iter).unwrap(), (15, 12));
    }
}
