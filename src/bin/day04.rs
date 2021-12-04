use aoc2021::parser;

pub fn main() {
    let s = std::fs::read_to_string("input/input04.txt").unwrap();
    let (s, input_numbers) = parser::signed_decimal_comma_separated_list(&s).unwrap();

    let mut boards = parser::bingo_board_list(s).unwrap().1;

    println!("{:?}", input_numbers);
    println!("{:?}", boards);

    for number in input_numbers {
        let mut remove = Vec::new();
        let last_board = boards.len() == 1;
        for (i, board) in boards.iter_mut().enumerate() {
            if board.apply(number) {
                println!("bingo!");
                println!("res: {}", board.count_remaining() * number);

                if last_board {
                    println!("last bingo!");
                    println!("res: {}", board.count_remaining() * number);
                    return;
                }
                remove.push(i);
            }
        }
        remove.reverse();
        for i in remove {
            boards.remove(i);
        }
    }
}
