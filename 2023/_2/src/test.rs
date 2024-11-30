use _2023_2::{solve, solve_one, Bag};

fn main() {
    const TEST_CASES: &[(&str, bool)] = &[
        (
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            true,
        ),
        (
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            true,
        ),
        (
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            false,
        ),
        (
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            false,
        ),
        (
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            true,
        ),
    ];

    const FINAL_RESULT: u32 = 8;

    const BAG: Bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    for (case, expected) in TEST_CASES {
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

    let res = solve(sv);

    println!(
        "{} => final result expected: {} actual {}",
        res == FINAL_RESULT,
        FINAL_RESULT,
        res
    );
}
