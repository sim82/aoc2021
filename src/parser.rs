use std::ops::RangeInclusive;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, multispace0, multispace1, one_of, space0, space1},
    combinator::{map, opt, recognize},
    multi::{count, many0, many1, many_m_n, separated_list0, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::{BingoBoard, SfNumber, Vec2, Vec3};

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

pub fn coord2d_list(input: &str) -> IResult<&str, Vec<Vec2>> {
    separated_list0(multispace0, coord2d)(input)
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

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum FoldInstruction {
    X(i64),
    Y(i64),
}

pub fn fold_instruction(input: &str) -> IResult<&str, FoldInstruction> {
    let (input, (fi, num)) = preceded(
        tag("fold along "),
        separated_pair(alt((tag("x"), tag("y"))), char('='), signed_decimal),
    )(input)?;
    Ok((
        input,
        match fi {
            "x" => FoldInstruction::X(num),
            "y" => FoldInstruction::Y(num),
            _ => panic!("bad direction in fold instruction"),
        },
    ))
}

pub fn fold_instruction_list(input: &str) -> IResult<&str, Vec<FoldInstruction>> {
    separated_list0(multispace1, fold_instruction)(input)
}

pub fn coords_and_fold(input: &str) -> IResult<&str, (Vec<Vec2>, Vec<FoldInstruction>)> {
    separated_pair(coord2d_list, multispace1, fold_instruction_list)(input)
}

#[test]
fn test_fold() {
    let (_, fi) = fold_instruction("fold along y=7").unwrap();
    assert_eq!(fi, FoldInstruction::Y(7));

    let (input, x) = coords_and_fold(
        "1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5",
    )
    .unwrap();
    println!("{:?}", x);
}

pub fn snailfish_number(input: &str) -> IResult<&str, SfNumber> {
    alt((
        map(signed_decimal, SfNumber::Number),
        delimited(
            char('['),
            map(
                separated_pair(snailfish_number, char(','), snailfish_number),
                |(l, r)| SfNumber::Pair(Box::new(l), Box::new(r)),
            ),
            char(']'),
        ),
    ))(input)
}
#[test]
fn snailfish_test() {
    let (_, mut x) =
        snailfish_number("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]").unwrap();
    println!("{:?}", x);
    let mut v = x.traverse_left_to_right_vec(0);
    println!("{:?}", v);

    match v.get_mut(2).unwrap() {
        (SfNumber::Number(x), _) => *x += 666,
        _ => todo!(),
    }
    println!("{:?}", v);

    let (_, mut x) = snailfish_number("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
    // loop {
    //     {
    //         let mut v = x.traverse_left_to_right_vec(0);
    //         println!("{:?}", v);

    //         if let Some((explode_pos, _)) = v
    //             .windows(2)
    //             .find_position(|x| x[0].1 == x[1].1 && x[0].1 >= 5)
    //         {
    //             println!("explode: {:?}", explode_pos);
    //             // *v[explode_pos].0 = SfNumber::Exploded;
    //             // *v[explode_pos + 1].0 = SfNumber::Exploded;

    //             if explode_pos > 0 {
    //                 let n = match (&v[explode_pos - 1].0, &v[explode_pos].0) {
    //                     (SfNumber::Number(a), SfNumber::Number(b)) => a + b,
    //                     _ => panic!("bad nodes"),
    //                 };
    //                 *v[explode_pos - 1].0 = SfNumber::Number(n);
    //             }
    //             if explode_pos < v.len() - 1 {
    //                 let n = match (&v[explode_pos + 1].0, &v[explode_pos + 2].0) {
    //                     (SfNumber::Number(a), SfNumber::Number(b)) => a + b,
    //                     _ => panic!("bad nodes"),
    //                 };
    //                 *v[explode_pos + 2].0 = SfNumber::Number(n);
    //             }
    //             *v[explode_pos].0 = SfNumber::Exploded;
    //             *v[explode_pos + 1].0 = SfNumber::Exploded;
    //             x.prune_exploded();
    //             continue;
    //         } else if x.split() {
    //             println!("split");
    //             continue;
    //         }
    //     }
    //     break;
    // }
    x.reduce();
    println!("v: {:?}", x);
    // *explode[0].0 = SfNumber::Exploded;
    // let (a, b) = explode.split_first_mut().unwrap();
    // let (b,_) -
}

pub fn scanner_head(input: &str) -> IResult<&str, i64> {
    let (input, num) = delimited(
        delimited(multispace0, tag("--- scanner"), multispace1),
        signed_decimal,
        delimited(multispace1, tag("---"), multispace0),
    )(input)?;
    Ok((input, num))
}

pub fn coord3d(input: &str) -> IResult<&str, Vec3> {
    let (input, (x, _, y, _, z)) = tuple((
        signed_decimal,
        char(','),
        signed_decimal,
        char(','),
        signed_decimal,
    ))(input)?;

    Ok((input, Vec3 { x, y, z }))
}

pub fn coord3d_list(input: &str) -> IResult<&str, Vec<Vec3>> {
    separated_list0(multispace0, coord3d)(input)
}

pub fn scanner(input: &str) -> IResult<&str, (i64, Vec<Vec3>)> {
    let (input, num) = scanner_head(input)?;
    let (input, coords) = coord3d_list(input)?;

    Ok((input, (num, coords)))
}

pub fn scanner_list(input: &str) -> IResult<&str, Vec<(i64, Vec<Vec3>)>> {
    separated_list0(multispace0, scanner)(input)
}

#[test]
fn test_scanner() {
    let (input, sl) = scanner_list(
        "--- scanner 0 ---
    404,-588,-901
    528,-643,409
    
    --- scanner 1 ---
    686,422,578
    605,423,415
    515,917,-361
    
    --- scanner 2 ---
    649,640,665
    682,-795,504
    -784,533,-524
    -644,584,-595
    ",
    )
    .unwrap();

    assert_eq!(sl.len(), 3);
    assert_eq!(sl[0].0, 0);
    assert_eq!(sl[1].0, 1);
    assert_eq!(sl[2].0, 2);

    assert_eq!(sl[0].1.len(), 2);
    assert_eq!(sl[1].1.len(), 3);
    assert_eq!(sl[2].1.len(), 4);
    assert_eq!(
        sl[0].1[0],
        Vec3 {
            x: 404,
            y: -588,
            z: -901
        }
    );
    assert_eq!(
        sl[1].1[1],
        Vec3 {
            x: 605,
            y: 423,
            z: 415
        }
    );
    assert_eq!(
        sl[2].1[2],
        Vec3 {
            x: -784,
            y: 533,
            z: -524
        }
    );
}

pub fn range(input: &str) -> IResult<&str, RangeInclusive<i64>> {
    map(
        separated_pair(signed_decimal, tag(".."), signed_decimal),
        |(l, r)| l..=r,
    )(input)
}
pub fn named_range(input: &str) -> IResult<&str, (&str, RangeInclusive<i64>)> {
    separated_pair(alpha1, tag("="), range)(input)
}

type RebootStep = (
    bool,
    RangeInclusive<i64>,
    RangeInclusive<i64>,
    RangeInclusive<i64>,
);

pub fn reboot_step(input: &str) -> IResult<&str, RebootStep> {
    let (input, (on_off, ranges)) = separated_pair(
        alt((tag("on"), tag("off"))),
        space1,
        separated_list1(tag(","), named_range),
    )(input)?;

    let on_off = match on_off {
        "on" => true,
        "off" => false,
        _ => panic!("unhandled on_off"),
    };

    assert_eq!(ranges[0].0, "x");
    assert_eq!(ranges[1].0, "y");
    assert_eq!(ranges[2].0, "z");

    Ok((
        input,
        (
            on_off,
            ranges[0].1.clone(),
            ranges[1].1.clone(),
            ranges[2].1.clone(),
        ),
    ))
}

pub fn reboot_step_list(input: &str) -> IResult<&str, Vec<RebootStep>> {
    separated_list1(multispace1, reboot_step)(input)
}

#[test]
fn test_reboot_step() {
    assert_eq!(
        reboot_step("on x=-22..26,y=-27..20,z=-29..19").unwrap(),
        ("", (true, -22..=26, -27..=20, -29..=19))
    )
}
