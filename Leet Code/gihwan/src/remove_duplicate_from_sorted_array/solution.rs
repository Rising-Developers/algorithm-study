const INVALID: i32 = 1000;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let nums_clone = nums.clone();

    nums.clear();

    let mut count = 0;

    for num in nums_clone {
        if num != *nums.last().unwrap_or_else(|| &INVALID) {
            nums.push(num);
            count += 1;
        }
    }

    count
}

pub fn best_solution_at_leetcode(nums: &mut Vec<i32>) -> i32 {
    nums.dedup(); // vector 에서 반복되는 값을 제거해주는 메서드
    nums.len() as i32
}
