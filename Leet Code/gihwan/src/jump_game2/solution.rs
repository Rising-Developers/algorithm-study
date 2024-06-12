const MAX: i32 = 10000;

pub fn jump(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = nums.iter().map(|_| MAX).collect();

    let mut index = 0;

    dp[0] = 0;

    while index < dp.len() {
        let jump_amount = nums[index];

        update_dp(&mut dp, index, jump_amount);

        index += 1;
    }

    *dp.last().unwrap()
}

fn update_dp(dp: &mut Vec<i32>, index: usize, jump_amount: i32) {
    let mut count = 0;
    let cur_val = dp[index] + 1;

    for memo in dp[index + 1..].iter_mut() {
        if count >= jump_amount {
            break;
        }

        if *memo > cur_val {
            *memo = cur_val
        }

        count += 1;
    }
}

#[test]
fn test_case_1() {
    let nums = vec![2, 3, 1, 1, 4];

    assert_eq!(jump(nums), 2);
}

#[test]
fn test_case_2() {
    let nums = vec![2, 3, 0, 1, 4];

    assert_eq!(jump(nums), 2);
}
