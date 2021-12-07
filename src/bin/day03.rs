fn main() {
    let s = std::fs::read_to_string("input/input03.txt").unwrap();
    let input = s
        .split_whitespace()
        .map(|s| {
            s.chars()
                .map(|c| if c == '0' { 0usize } else { 1usize })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum = input
        .iter()
        .cloned()
        .reduce(|a, v| {
            a.iter()
                .cloned()
                .zip(v.iter().cloned())
                .map(|(a, b)| a + b)
                .collect()
        })
        .unwrap();

    let gamma_bits = sum
        .iter()
        .map(|d| *d >= (input.len() / 2))
        .collect::<Vec<_>>();
    let gamma = bits_to_int(gamma_bits.iter().cloned());
    let delta = (!gamma) & 0b111111111111;
    println!(
        "{} {:?} {:?} {} {}",
        input.len(),
        sum,
        gamma,
        delta,
        gamma * delta
    );

    let input_bits = input
        .iter()
        .map(|v| v.iter().map(|b| *b == 1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut oxy = input_bits.clone();
    let mut co2 = input_bits;

    for i in 0..12 {
        // let is_gamma = gamma_bits[i];
        if oxy.len() > 1 {
            let num_true = oxy.iter().filter(|v| v[i]).count();
            let most_common = num_true * 2 >= oxy.len();
            // println!(
            //     "most common {} {:?} {} {}",
            //     i,
            //     most_common,
            //     num_true,
            //     oxy.len()
            // );

            oxy.retain(|v| v[i] == most_common);
        }
        if co2.len() > 1 {
            let num_true = co2.iter().filter(|v| v[i]).count();
            let least_common = num_true * 2 < co2.len();
            println!(
                "least common {} {:?} {} {}",
                i,
                least_common,
                num_true,
                co2.len()
            );

            co2.retain(|v| v[i] == least_common)
        }
    }
    println!("oxy: {:?}", oxy);
    println!("co2: {:?}", co2);
    println!(
        "res: {}",
        bits_to_int(oxy[0].iter().cloned()) * bits_to_int(co2[0].iter().cloned())
    );
}

fn bits_to_int<I: IntoIterator<Item = bool>>(i: I) -> usize {
    let mut acc = 0;
    for b in i.into_iter() {
        acc <<= 1;
        if b {
            acc += 1;
        }
    }

    acc
}

#[test]
pub fn test() {
    assert_eq!(bits_to_int([true, false]), 2);
}
