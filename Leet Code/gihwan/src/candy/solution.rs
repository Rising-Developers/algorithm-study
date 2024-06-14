/// # Candy
///
/// n명의 아이들이 한 줄로 서 있습니다. 각 아이에게는 정수 배열 ratings에 주어진 평점 값이 할당됩니다.
/// 다음 요구사항을 만족하며 아이들에게 사탕을 나눠주려 합니다:
///
/// 1.	각 아이는 적어도 하나의 사탕을 받아야 합니다.
/// 2.	높은 평점을 가진 아이는 이웃한 아이들보다 더 많은 사탕을 받아야 합니다.
///
/// 아이들에게 사탕을 나눠주기 위해 필요한 최소 사탕 수를 반환하세요.

struct Solution {}

struct _LeetCodeSolution {}

impl _LeetCodeSolution {
    pub fn _candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut candies = vec![1; n];

        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }

        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = std::cmp::max(candies[i], candies[i + 1] + 1);
            }
        }

        candies.iter().sum()
    }
}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candies: Vec<i32> = vec![1; ratings.len()];

        for index in 0..ratings.len() {
            if index as i32 - 1 >= 0 && ratings[index - 1] > ratings[index] {
                candies[index - 1] += 1;
            }

            if index + 1 < ratings.len() && ratings[index + 1] > ratings[index] {
                candies[index + 1] += 1;
            }
        }

        candies.into_iter().reduce(|acc, e| acc + e).unwrap()
    }
}

#[test]
fn test_case_1() {
    let ratings = vec![1, 0, 2];

    assert_eq!(Solution::candy(ratings), 5);
}

#[test]
fn test_case_2() {
    let ratings = vec![1, 2, 2];

    assert_eq!(Solution::candy(ratings), 4);
}
