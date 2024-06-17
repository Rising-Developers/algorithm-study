struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut target = num;

        let mut result = String::from("");

        while target > 0 {
            match target {
                val if val >= 1000 => {
                    target -= 1000;
                    result.push_str("M");
                }
                val if val >= 900 => {
                    target -= 900;
                    result.push_str("CM");
                }
                val if val >= 500 => {
                    target -= 500;
                    result.push_str("D");
                }
                val if val >= 400 => {
                    target -= 400;
                    result.push_str("CD");
                }
                val if val >= 100 => {
                    target -= 100;
                    result.push_str("C");
                }
                val if val >= 90 => {
                    target -= 90;
                    result.push_str("XC");
                }
                val if val >= 50 => {
                    target -= 50;
                    result.push_str("L");
                }
                val if val >= 40 => {
                    target -= 40;
                    result.push_str("XL");
                }
                val if val >= 10 => {
                    target -= 10;
                    result.push_str("X");
                }
                val if val >= 9 => {
                    target -= 9;
                    result.push_str("IX");
                }
                val if val >= 5 => {
                    target -= 5;
                    result.push_str("V");
                }
                val if val >= 4 => {
                    target -= 4;
                    result.push_str("IV");
                }
                _ => {
                    target -= 1;
                    result.push_str("I");
                }
            }
        }

        result
    }
}

#[test]
fn test_case_1() {
    let num = 3749;

    assert_eq!(Solution::int_to_roman(num), "MMMDCCXLIX");
}

#[test]
fn test_case_2() {
    let num = 58;

    assert_eq!(Solution::int_to_roman(num), "LVIII");
}

#[test]
fn test_case_3() {
    let num = 1994;

    assert_eq!(Solution::int_to_roman(num), "MCMXCIV");
}
