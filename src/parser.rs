use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1, one_of, space0},
    combinator::{opt, recognize},
    multi::{count, many0, many1, many_m_n, separated_list0, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::{BingoBoard, Vec2};

// use crate::{Claim, RecordTimestamp, RecordType, Rect};

pub fn float(input: &str) -> IResult<&str, &str> {
    alt((
        // Case one: .42
        recognize(tuple((
            char('.'),
            decimal,
            opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
        ))), // Case two: 42e42 and 42.42e42
        recognize(tuple((
            decimal,
            opt(preceded(char('.'), decimal)),
            one_of("eE"),
            opt(one_of("+-")),
            decimal,
        ))), // Case three: 42. and 42.42
        recognize(tuple((decimal, char('.'), opt(decimal)))),
    ))(input)
}

pub fn signed_decimal(input: &str) -> IResult<&str, i64> {
    alt((recognize(tuple((opt(one_of("+-")), decimal))), decimal))(input)
        .map(|x| (x.0, x.1.parse().unwrap()))
}

pub fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

pub fn signed_decimal_list(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list0(multispace1, signed_decimal)(input)
}

pub fn signed_decimal_comma_separated_list(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list0(char(','), signed_decimal)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubmarineCommand {
    Up(i64),
    Down(i64),
    Forward(i64),
}
pub fn submarine_command(input: &str) -> IResult<&str, SubmarineCommand> {
    fn up(input: &str) -> IResult<&str, SubmarineCommand> {
        preceded(tuple((tag("up"), multispace1)), signed_decimal)(input)
            .map(|(input, i)| (input, SubmarineCommand::Up(i)))
    }
    fn down(input: &str) -> IResult<&str, SubmarineCommand> {
        preceded(tuple((tag("down"), multispace1)), signed_decimal)(input)
            .map(|(input, i)| (input, SubmarineCommand::Down(i)))
    }
    fn forward(input: &str) -> IResult<&str, SubmarineCommand> {
        preceded(tuple((tag("forward"), multispace1)), signed_decimal)(input)
            .map(|(input, i)| (input, SubmarineCommand::Forward(i)))
    }
    alt((up, down, forward))(input)
}

pub fn submarine_command_list(input: &str) -> IResult<&str, Vec<SubmarineCommand>> {
    separated_list0(multispace1, submarine_command)(input)
}

#[test]
fn submarine_test() {
    assert_eq!(
        submarine_command("up 10").unwrap(),
        ("", SubmarineCommand::Up(10))
    );
}

pub fn bingo_board(input: &str) -> IResult<&str, BingoBoard> {
    fn bingo_line(input: &str) -> IResult<&str, Vec<i64>> {
        count(delimited(multispace0, signed_decimal, multispace0), 5)(input)
    }
    let (input, lines) = count(delimited(multispace0, bingo_line, multispace0), 5)(input)?;
    Ok((input, BingoBoard::new(lines)))
}

pub fn bingo_board_list(input: &str) -> IResult<&str, Vec<BingoBoard>> {
    many0(bingo_board)(input)
}

#[test]
fn bingo_test() {
    println!(
        "{:?}",
        bingo_board_list(
            "

88 29 95 98 57
49 36  6 23 83
18  5 45 40 44
62 81 74 99 87
46 56 35 21 52


49 11 72 87 56
40 94 71 70  3
65  2 90 64 63
32 79 24 44 55
58 53 35 77 60

    "
        )
    )
}

pub fn coord2d(input: &str) -> IResult<&str, Vec2> {
    let (input, (x, y)) = separated_pair(signed_decimal, char(','), signed_decimal)(input)?;

    Ok((input, Vec2 { x, y }))
}

pub fn line_segment(input: &str) -> IResult<&str, (Vec2, Vec2)> {
    let (input, (p1, p2)) = separated_pair(coord2d, tag(" -> "), coord2d)(input)?;
    Ok((input, (p1, p2)))
}

pub fn line_segment_list(input: &str) -> IResult<&str, Vec<(Vec2, Vec2)>> {
    separated_list0(multispace1, line_segment)(input)
}

#[test]
fn line_test() {
    assert_eq!(
        line_segment("0,9 -> 5,9").unwrap(),
        ("", (Vec2 { x: 0, y: 9 }, Vec2 { x: 5, y: 9 }))
    );
}

pub fn seven_segment_sample(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    fn ag_string(input: &str) -> IResult<&str, &str> {
        recognize(many1(one_of("abcdefg")))(input)
    }

    fn ag_string_list_10(input: &str) -> IResult<&str, Vec<&str>> {
        many_m_n(10, 10, delimited(space0, ag_string, space0))(input)
    }

    fn ag_string_list_4(input: &str) -> IResult<&str, Vec<&str>> {
        many_m_n(4, 4, delimited(space0, ag_string, space0))(input)
    }

    separated_pair(
        ag_string_list_10,
        delimited(multispace0, tag("|"), multispace0),
        ag_string_list_4,
    )(input)
}

pub fn seven_segment_sample_list(input: &str) -> IResult<&str, Vec<(Vec<&str>, Vec<&str>)>> {
    separated_list0(multispace1, seven_segment_sample)(input)
}

#[test]
fn test_seven_segment() {
    // fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
    // gebdcfa ecba ca fadegcb
    assert_eq!(
        seven_segment_sample(
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"
        )
        .unwrap(),
        (
            "",
            (
                vec![
                    "fgeab", "ca", "afcebg", "bdacfeg", "cfaedg", "gcfdb", "baec", "bfadeg",
                    "bafgc", "acf"
                ],
                vec!["gebdcfa", "ecba", "ca", "fadegcb"]
            )
        )
    );

    assert_eq!(
        seven_segment_sample_list(
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"
        )
        .unwrap(),
        (
            "",
            vec![
                (
                    vec![
                        "fgeab", "ca", "afcebg", "bdacfeg", "cfaedg", "gcfdb", "baec", "bfadeg",
                        "bafgc", "acf"
                    ],
                    vec!["gebdcfa", "ecba", "ca", "fadegcb"]
                ),
                (
                    vec![
                        "fgeab", "ca", "afcebg", "bdacfeg", "cfaedg", "gcfdb", "baec", "bfadeg",
                        "bafgc", "acf"
                    ],
                    vec!["gebdcfa", "ecba", "ca", "fadegcb"]
                ),
            ]
        )
    );
}
