pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return true;
    }

    let mut dp: Vec<bool> = nums.iter().map(|item| *item != 0).collect();

    for i in 0..nums.len() {
        if nums[i] != 0 {
            let jump_amount = nums[i] as usize;
            update_dp(i, i + jump_amount, &mut dp);
        }
    }

    if let Some(val) = dp.last_mut() {
        *val = true;
    }

    dp.iter().all(|memo| *memo)
}

fn update_dp(start: usize, end: usize, dp: &mut Vec<bool>) {
    let mut index = start;

    loop {
        if index >= end || index >= dp.len() {
            return;
        }

        dp[index] = true;

        index += 1;
    }
}

#[test]
fn test_case_1() {
    let nums = vec![2, 3, 1, 1, 4];

    assert_eq!(can_jump(nums), true);
}

#[test]
fn test_case_2() {
    let nums = vec![3, 2, 1, 0, 4];

    assert_eq!(can_jump(nums), false);
}
