use std::cmp::Ordering;

use utils::{get_input_path, parse_file_into};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn new_from(hand_values: &[usize; 5]) -> Self {
        let mut pairs: [usize; 15] = [0; 15];

        for value in hand_values.iter() {
            pairs[*value] += 1;
        }

        if pairs.contains(&5) {
            return HandType::FiveOfAKind;
        }

        if pairs.contains(&4) {
            return HandType::FourOfAKind;
        }

        if pairs.contains(&3) && pairs.contains(&2) {
            return HandType::FullHouse;
        }

        if pairs.contains(&3) {
            return HandType::ThreeOfAKind;
        }

        if pairs.iter().filter(|&v| v == &2).count() == 2 {
            return HandType::TwoPair;
        }

        if pairs.contains(&2) {
            return HandType::OnePair;
        }

        HandType::HighCard
    }

    fn new_from_with_jokers(hand_values: &[usize; 5]) -> HandType {
        let mut pairs: [usize; 15] = [0; 15];

        for value in hand_values.iter() {
            pairs[*value] += 1;
        }

        let jokers = pairs[0];
        pairs[0] = 0;

        if pairs.contains(&5) {
            return HandType::FiveOfAKind;
        }

        if pairs.contains(&4) {
            if jokers == 1 {
                return HandType::FiveOfAKind;
            }
            return HandType::FourOfAKind;
        }

        if pairs.contains(&3) && pairs.contains(&2) {
            return HandType::FullHouse;
        }

        if pairs.contains(&3) {
            match jokers {
                0 => {
                    return HandType::ThreeOfAKind;
                }
                1 => {
                    return HandType::FourOfAKind;
                }
                2 => {
                    return HandType::FiveOfAKind;
                }
                _ => panic!("Sould not be here."),
            }
        }

        if pairs.iter().filter(|&v| v == &2).count() == 2 {
            if jokers == 1 {
                return HandType::FullHouse;
            }
            return HandType::TwoPair;
        }

        if pairs.contains(&2) {
            match jokers {
                0 => {
                    return HandType::OnePair;
                }
                1 => {
                    return HandType::ThreeOfAKind;
                }
                2 => {
                    return HandType::FourOfAKind;
                }
                3 => {
                    return HandType::FiveOfAKind;
                }
                _ => panic!("Sould not be here."),
            }
        }

        match jokers {
            0 => {
                return HandType::HighCard;
            }
            1 => {
                return HandType::OnePair;
            }
            2 => {
                return HandType::ThreeOfAKind;
            }
            3 => {
                return HandType::FourOfAKind;
            }
            4 => {
                return HandType::FiveOfAKind;
            }
            5 => {
                return HandType::FiveOfAKind;
            }
            _ => panic!("Sould not be here."),
        }
    }
}

#[derive(Debug)]
struct Hand {
    #[allow(dead_code)]
    hand: String,
    hand_values: [usize; 5],
    bid: usize,
    hand_type: HandType,
}

fn into_hand_values(c: char) -> usize {
    match c {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Should not be here."),
    }
}

impl From<String> for Hand {
    fn from(value: String) -> Self {
        let mut split = value.split(' ');
        let hand: String = split.next().unwrap().into();
        let bid = split.next().unwrap().parse().unwrap();

        let mut hand_values = [0; 5];

        for (i, v) in hand.chars().map(into_hand_values).enumerate() {
            hand_values[i] = v;
        }

        let hand_type = HandType::new_from(&hand_values);

        Self {
            hand,
            hand_values,
            bid,
            hand_type: hand_type,
        }
    }
}

fn sort_hands(left: &Hand, right: &Hand) -> Ordering {
    match left.hand_type.cmp(&right.hand_type) {
        Ordering::Equal => {}
        ord => {
            return ord;
        }
    }

    for i in 0..4 {
        match left.hand_values[i].cmp(&right.hand_values[i]) {
            Ordering::Equal => {}
            ord => {
                return ord;
            }
        }
    }

    left.hand_values[4].cmp(&right.hand_values[4])
}

#[derive(Debug)]
struct JokerHand {
    #[allow(dead_code)]
    hand: String,
    hand_values: [usize; 5],
    bid: usize,
    hand_type: HandType,
}

fn into_hand_values_joker(c: char) -> usize {
    match c {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 0,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Should not be here."),
    }
}

impl From<String> for JokerHand {
    fn from(value: String) -> Self {
        let mut split = value.split(' ');
        let hand: String = split.next().unwrap().into();
        let bid = split.next().unwrap().parse().unwrap();

        let mut hand_values = [0; 5];

        for (i, v) in hand.chars().map(into_hand_values_joker).enumerate() {
            hand_values[i] = v;
        }

        let hand_type = HandType::new_from_with_jokers(&hand_values);

        Self {
            hand,
            hand_values,
            bid,
            hand_type: hand_type,
        }
    }
}

fn sort_hands_joker(left: &JokerHand, right: &JokerHand) -> Ordering {
    match left.hand_type.cmp(&right.hand_type) {
        Ordering::Equal => {}
        ord => {
            return ord;
        }
    }

    for i in 0..4 {
        match left.hand_values[i].cmp(&right.hand_values[i]) {
            Ordering::Equal => {}
            ord => {
                return ord;
            }
        }
    }

    left.hand_values[4].cmp(&right.hand_values[4])
}

fn run(input_file: &str) {
    // Preamble
    // Parse
    let mut values: Vec<Hand> = parse_file_into::<Hand>(input_file);

    values.sort_by(sort_hands);

    // Solve
    let mut result = 0;

    for (idx, hand) in values.iter().enumerate() {
        result += (idx + 1) * hand.bid;
    }

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    // Parse
    let mut values: Vec<JokerHand> = parse_file_into::<JokerHand>(input_file);

    values.sort_by(sort_hands_joker);

    // Solve
    let mut result = 0;

    for (idx, hand) in values.iter().enumerate() {
        result += (idx + 1) * hand.bid;
    }

    // Result
    println!("Result of part 1 is {}", result);
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    println!("{:?}", input_file);

    run(input_file);
    run2(input_file);
}

#[cfg(test)]
mod main_test {
    use utils::get_test_input_path;

    use crate::run;
    use crate::run2;

    #[test]
    fn test_input_part_1() {
        let input_path = get_test_input_path(file!());
        run(input_path.to_str().unwrap());
    }

    #[test]
    fn test_input_part_2() {
        let input_path = get_test_input_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
