struct _LeetCodeSolution {}

impl _LeetCodeSolution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result = String::from("");
        let reversed_s: Vec<String> = s
            .split_whitespace()
            .rev()
            .map(|str| String::from(str))
            .collect();

        for (index, word) in reversed_s.iter().enumerate() {
            result.push_str(word);
            if index == reversed_s.len() - 1 {
                continue;
            }
            result.push(' ');
        }

        result
    }
}

#[test]
fn test_case_1() {
    let s = String::from("the sky is blue");

    assert_eq!(Solution::reverse_words(s), String::from("blue is sky the"));
}
