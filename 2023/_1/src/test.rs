use _2023_1::{solve, solve_one};

fn main() {
    const TEST_CASES: &[(&str, u8)] = &[
        ("1abc2", 12),
        ("pqr3stu8vwx", 38),
        ("a1b2c3d4e5f", 15),
        ("treb7uchet", 77),
        ("7", 77),
    ];

    const FINAL_RESULT: u32 = 219;

    for (case, expected) in TEST_CASES {
        let result = solve_one(case);
        let is_ok = &result == expected;
        println!(
            "{} ==> case: {} expected: {} actual: {}",
            is_ok, case, expected, result
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
