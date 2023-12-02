use nom::{
    IResult,
    sequence::{delimited, separated_pair},
    branch::alt,
    multi::separated_list,
    character::complete::{char, u32, space1},
    bytes::complete::{tag}
};

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

enum ColorCount {
    Blue(u32),
    Red(u32),
    Green(u32),
}

type Round = Vec<ColorCount>;

struct Game {
    id: GameId,
    rounds: Vec<Round>
}

struct GameId(u32);

fn score_game(game: &Game) -> u32 {
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

fn game_id(input: &str) -> IResult<&str, GameId> {
    match delimited(tag("Game"), u32, char(':'))(input) {
        Err(e) => Err(e),
        Ok((rest, id)) => Ok((rest, GameId(id))),
    }
}

fn blue(input: &str) -> IResult<&str, ColorCount> {
    let (rest, (count, _)) = separated_pair(u32, space1, tag("blue"))(input)?;
    Ok((rest, ColorCount::Blue(count)))
}

fn red(input: &str) -> IResult<&str, ColorCount> {
    let (rest, (count, _)) = separated_pair(u32, space1, tag("red"))(input)?;
    Ok((rest, ColorCount::Red(count)))
}

fn green(input: &str) -> IResult<&str, ColorCount> {
    let (rest, (count, _)) = separated_pair(u32, space1, tag("green"))(input)?;
    Ok((rest, ColorCount::Green(count)))
}

fn color_count(input: &str) -> IResult<&str, ColorCount> {
    alt((blue, green, red))(input)
}

fn record(input: &str) -> IResult<&str, Record> {
    sepa
}

fn main() {
    println!("Hello, world!");
}
