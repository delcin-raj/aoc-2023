use nom::{
    IResult,
    sequence::{delimited, separated_pair},
    branch::alt,
    multi::separated_list0,
    character::complete::{char, u64, space1, space0},
    bytes::complete::tag,
};
use std::fs;
use std::io::{self};

const RED: u64 = 12;
const GREEN: u64 = 13;
const BLUE: u64 = 14;

#[derive(PartialEq, Eq, Debug)]
enum ColorCount {
    Blue(u64),
    Red(u64),
    Green(u64),
}

type Round = Vec<ColorCount>;

#[derive(PartialEq, Eq, Debug)]
struct Game {
    id: GameId,
    rounds: Vec<Round>
}

#[derive(PartialEq, Eq, Debug)]
struct GameId(u64);

fn game_id(input: &str) -> IResult<&str, GameId> {
    match delimited(tag("Game "), u64, char(':'))(input) {
        Err(e) => Err(e),
        Ok((rest, id)) => Ok((rest, GameId(id))),
    }
}

fn blue(input: &str) -> IResult<&str, ColorCount> {
    let (input, _) = space0(input)?;
    let (rest, (count, _)) = separated_pair(u64, space1, tag("blue"))(input)?;
    Ok((rest, ColorCount::Blue(count)))
}

fn red(input: &str) -> IResult<&str, ColorCount> {
    let (input, _) = space0(input)?;
    let (rest, (count, _)) = separated_pair(u64, space1, tag("red"))(input)?;
    Ok((rest, ColorCount::Red(count)))
}

fn green(input: &str) -> IResult<&str, ColorCount> {
    let (input, _) = space0(input)?;
    let (rest, (count, _)) = separated_pair(u64, space1, tag("green"))(input)?;
    Ok((rest, ColorCount::Green(count)))
}

fn color_count(input: &str) -> IResult<&str, ColorCount> {
    alt((blue, green, red))(input)
}

fn round(input: &str) -> IResult<&str, Round> {
    let (input, _) = space0(input)?;
    separated_list0(char(','), color_count)(input)
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input1, id) = game_id(input)?;
    let (input2, rounds) = separated_list0(char(';'), round)(input1)?;
    Ok((input2, Game{id, rounds}))
}

fn games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list0(char('\n'), game)(input)
}

fn score_game(game: &Game) -> u64 {
    for r in game.rounds.iter() {
        for b in r {
            match b {
                ColorCount::Red(v) => if *v > RED {return 0;},
                ColorCount::Green(v) => if *v > GREEN {return 0;},
                ColorCount::Blue(v) => if *v > BLUE {return 0;},
            }
        }
    }
    game.id.0
}

fn min_cubes(rounds: &Vec<Round>) -> u64 {
    let (mut r, mut b, mut g) = (0, 0 ,0);
    for round in rounds.iter() {
        for c in round.iter() {
            match c {
                ColorCount::Red(v) => if *v > r {
                    r = *v;
                },
                ColorCount::Blue(v) => if *v > b {
                    b = *v;
                },
                ColorCount::Green(v) => if *v > g {
                    g = *v;
                },
            }
        }
    }
    r * b * g
}

fn main() -> io::Result<()> {
    let content = fs::read_to_string("input/data.dat")?;
    match games(&content) {
        Err(e) => println!("oh oh {e}"),
        Ok((_, games)) => {
            let res: u64 = games.iter().map(score_game).sum();
            println!("{res}");

            let res: u64 = games.iter().map(|r| min_cubes(&r.rounds)).sum();
            println!("{res}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_id() {
        assert_eq!(game_id("Game 11: 5 blue, 6 red, 10 green"),
        Ok((" 5 blue, 6 red, 10 green", GameId(11)))
    );
    }

    #[test]
    fn test_color() {
        assert_eq!(blue("5 blue, 6 red"),
        Ok((", 6 red", ColorCount::Blue(5)))
    );
    }
}
