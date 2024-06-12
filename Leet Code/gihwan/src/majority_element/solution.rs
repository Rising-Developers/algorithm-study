use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    let mut max_count = -1;
    let mut majority_number = 0;

    for num in nums {
        let count = hash_map.entry(num).or_insert(0);

        *count += 1;

        if *count > max_count {
            max_count = *count;
            majority_number = num;
        }
    }

    majority_number
}

#[test]
fn test_case_1() {
    let nums = vec![3, 2, 3];

    let result = majority_element(nums);

    assert_eq!(result, 3);
}

#[test]
fn test_case_2() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];

    let result = majority_element(nums);

    assert_eq!(result, 2);
}
