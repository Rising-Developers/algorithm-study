pub struct Solution {}

struct _LeetCodeSolution {}

impl _LeetCodeSolution {
    pub fn _longest_common_prefix(input: Vec<String>) -> String {
        input
            .into_iter()
            // reduce 는 acc 의 초기값이 첫 번째 요소다.
            // fold 는 첫 번째 인수를 초기값으로 받는다.
            .reduce(|acc, cur| {
                acc.chars()
                    // 두 이터레이터를 합쳐서 튜플의 리스트로 만든다.
                    // [1, 2, 3].zip([4, 5, 6]) => [(1, 4), (2, 5), (3, 6)]
                    .zip(cur.chars())
                    // 클로저의 반환값이 true 인 동안 iterator 를 만든다
                    // false 를 반환하면 중단한다
                    .take_while(|(a, c)| a == c)
                    .map(|(c, _)| c)
                    .collect()
            })
            .unwrap()
    }
}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let shortest_str = Self::find_shortest_str(&strs);
        let mut result = String::from("");

        for (cur_index, c) in shortest_str.chars().enumerate() {
            if strs
                .iter()
                .all(|target| target.chars().nth(cur_index).unwrap() == c)
            {
                result.push(c);
            } else {
                break;
            }
        }

        result
    }

    fn find_shortest_str(strs: &Vec<String>) -> String {
        let mut index = 0;
        let mut length = 2000;

        for (cur_index, str) in strs.iter().enumerate() {
            if str.len() < length {
                length = str.len();
                index = cur_index;
            }
        }

        strs[index].clone()
    }
}

#[test]
fn test_case_1() {
    let result = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];

    assert_eq!(Solution::longest_common_prefix(result), String::from("fl"));
}
