use _2023_2::{min_bag_power, solve, solve_one, Bag};

fn main() {
    const TEST_CASES: &[(&str, bool, u32)] = &[
        (
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            true,
            48,
        ),
        (
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            true,
            12,
        ),
        (
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            false,
            1560,
        ),
        (
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            false,
            630,
        ),
        (
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            true,
            36,
        ),
    ];

    const FINAL_RESULT: u32 = 8;

    const BAG: Bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    for (case, expected, _) in TEST_CASES {
        let result = solve_one(&BAG, case);
        let is_ok = &result.is_some() == expected;
        println!(
            "{} ==> case: {} expected: {} actual: {}",
            is_ok,
            case,
            expected,
            result.is_some()
        );
    }

    let sv: Vec<&str> = TEST_CASES.iter().map(|x| x.0).collect();

    let res = solve(&sv);

    println!(
        "{} => final result expected: {} actual {}",
        res == FINAL_RESULT,
        FINAL_RESULT,
        res
    );

    println!("Part 2");
    for (case, _, expected) in TEST_CASES {
        let result = min_bag_power(case);
        let is_ok = &result == expected;
        println!(
            "{} ==> case: {} expected: {} actual: {}",
            is_ok, case, expected, result
        );
    }
}
