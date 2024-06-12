/// # Product of array except self
///
/// 정수 배열 nums가 주어졌을 때, answer[i]가 nums[i]를 제외한 nums의 모든 요소의 곱과 같도록 배열 답을 반환합니다.
/// nums의 접두사 또는 접미사의 곱은 32비트 정수에 맞도록 보장됩니다.
/// 나누기 연산을 사용하지 않고 O(n) 시간 내에 실행되는 알고리즘을 작성해야 합니다.
///
/// ## Hint 1
/// 접두사와 접미사 곱을 효율적으로 활용하여 각 인덱스에 대해 자기를 제외한 모든 요소의 곱을 계산할 수 있는 방법을 생각해 보세요.
/// 중복 계산을 피하기 위해 접두사와 접미사의 곱을 선형 시간 내에 미리 계산할 수 있나요?

struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut left_dp: Vec<i32> = Vec::new();
        let mut right_dp: Vec<i32> = Vec::new();

        let mut nums_for_right_dp = nums.clone();
        nums_for_right_dp.reverse();

        Self::cal_dp(&nums, &mut left_dp);
        Self::cal_dp(&nums_for_right_dp, &mut right_dp);

        right_dp.reverse();

        Self::cal_product(left_dp, right_dp, nums.len())
    }

    fn cal_dp(nums: &Vec<i32>, dp: &mut Vec<i32>) {
        for (index, val) in nums.iter().enumerate() {
            let last = dp.last().unwrap_or_else(|| &nums[0]);
            let cur = if index == 0 { *last } else { last * val };

            dp.push(cur);
        }
    }

    fn cal_product(left_dp: Vec<i32>, right_dp: Vec<i32>, length: usize) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        result.push(right_dp[1]);

        for i in 1..length {
            let product = left_dp.get(i - 1).unwrap_or_else(|| &1)
                * right_dp.get(i + 1).unwrap_or_else(|| &1);
            result.push(product);
        }

        result
    }
}

#[test]
fn test_case_1() {
    let input = vec![1, 2, 3, 4];

    let output = vec![24, 12, 8, 6];

    assert_eq!(Solution::product_except_self(input), output);
}

#[test]
fn test_case_2() {
    let input = vec![-1, 1, 0, -3, 3];

    let output = vec![0, 0, 9, 0, 0];

    assert_eq!(Solution::product_except_self(input), output);
}
