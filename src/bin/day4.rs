use nom::{
    bytes::complete::tag,
    character::complete::{char, space0, space1, u32},
    multi::separated_list0,
    sequence::{delimited, pair, separated_pair, preceded},
    IResult,
};
use std::fs;
use std::io::{self};

type Numbers = Vec<u32>;

#[derive(PartialEq, Eq, Debug)]
struct Card {
    id: usize,
    left: Numbers,
    right: Numbers,
}

fn card_id(input: &str) -> IResult<&str, u32> {
    delimited(pair(tag("Card"), space1), u32, char(':'))(input)
}

fn numbers(input: &str) -> IResult<&str, Numbers> {
    preceded(space0, separated_list0(space1, u32))(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    let (rest, id) = card_id(input)?;
    let (rest, (left, right)) = separated_pair(numbers, tag(" | "), numbers)(rest)?;
    Ok((rest, Card{id: id as usize, left, right}))
}

fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list0(char('\n'), card)(input)
}

fn main()-> io::Result<()> {
    let content = fs::read_to_string("input/day4.dat")?;
    let mut res: u128 = 0;
    const TWO: u128 = 2;
    match cards(&content) {
        Err(e) => println!("oh oh {e}"),
        Ok((_, cards)) => {
            let mut card_count: Vec<u128> = vec![1;cards.len() + 1];
            for card in cards.iter() {
                let mut wins = 0;
                for n in card.left.iter() {
                    if card.right.contains(&n) {
                        wins += 1;
                    }
                }
                if wins > 0 {
                    res += TWO.pow(wins - 1);
                    let left = card.id + 1;
                    let right = card.id + 1 + wins as usize;

                    for i in left..right {
                        if i >= card_count.len() {break;}
                        card_count[i] += card_count[card.id];
                    }
                }
            }
            println!("part 2 {}", card_count.iter().sum::<u128>() - 1);
        }
    }
    println!("part1: {res}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const one: &str = "Card   1: 33 34 29 52 91  7 31 42  2  6 | 53 52  6 96 42 91  2 23  7 38 90 28 31 51  1 26 33 22 95 34 29 77 32 86  3";

    #[test]
    fn test_card() {
        let rest = " 33 34 29 52 91  7 31 42  2  6 | 53 52  6 96 42 91  2 23  7 38 90 28 31 51  1 26 33 22 95 34 29 77 32 86  3";
        let res = numbers(rest);
        println!("{:?}", res);
        println!("{:?}", card(one));
    }

    #[test]
    fn test_card_id() {
        println!("{:?}", card_id(one));
    }

    #[test]
    fn test_numbers() {
        println!("{:?}", numbers(" 33 34 29 52 91  7 31 42  2  6 "));
    }
}