pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let nums_clone = nums.clone();

    let divider = nums_clone.len() as i32 - (k % nums.len() as i32);

    if divider < 0 {
        return;
    }

    let divider = divider as usize;

    nums.clear();

    let mut front = make_new_vector_from_slice(&nums_clone[..divider]);

    let mut back = make_new_vector_from_slice(&nums_clone[divider..]);

    insert_slice(&mut back, nums);
    insert_slice(&mut front, nums);
}

fn make_new_vector_from_slice(other: &[i32]) -> Vec<i32> {
    let mut new_vec = Vec::new();
    new_vec.extend_from_slice(other);
    new_vec
}

fn insert_slice(slice: &mut Vec<i32>, nums: &mut Vec<i32>) {
    nums.append(slice);
}

#[test]
fn test_case_1() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;

    rotate(&mut nums, k);

    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
}

#[test]
fn test_case_2() {
    let mut nums = vec![-1, -100, 3, 99];
    let k = 2;

    rotate(&mut nums, k);

    assert_eq!(nums, vec![3, 99, -1, -100]);
}

#[test]
fn test_case_3() {
    let mut nums = vec![-1];
    let k = 2;

    rotate(&mut nums, k);

    assert_eq!(nums, vec![-1]);
}

pub fn leetcode_best_solution(nums: &mut Vec<i32>, k: i32) {
    let divider = k as usize % nums.len();

    nums.reverse();
    nums[..divider].reverse();
    nums[divider..].reverse();
}
