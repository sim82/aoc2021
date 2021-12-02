use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1, one_of},
    combinator::{opt, recognize},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

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
