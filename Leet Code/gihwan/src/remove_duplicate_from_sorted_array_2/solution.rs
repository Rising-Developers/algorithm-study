pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let last: i32 = nums.len() as i32 - 2;
    let mut i: usize = 0;

    if last <= 0 {
        return nums.len() as i32;
    }

    let mut last = last as usize;

    while i < last {
        let mut is_swapped = false;

        if i >= last {
            return nums.len() as i32;
        }

        let first = nums.get(i).unwrap();
        let second = nums.get(i + 1).unwrap();
        let third = nums.get(i + 2).unwrap();

        if three_element_is_same(*first, *second, *third) {
            swap_all_element(nums, i, nums.len() - 1);
            nums.pop();
            last -= 1;
            is_swapped = true;
        }

        if !is_swapped {
            i += 1;
        }
    }

    nums.len() as i32
}

fn three_element_is_same(first: i32, second: i32, third: i32) -> bool {
    first == second && second == third
}

fn swap_all_element(nums: &mut Vec<i32>, start: usize, end: usize) {
    for index in start..end {
        nums.swap(index, index + 1);
    }
}

#[test]
fn test_case_1() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];

    let result_nums = vec![1, 1, 2, 2, 3];

    let result = remove_duplicates(&mut nums);

    assert_eq!(result, 5);

    assert_eq!(nums, result_nums);
}

#[test]
fn test_case_2() {
    let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];

    let result_nums = vec![0, 0, 1, 1, 2, 3, 3];

    let result = remove_duplicates(&mut nums);

    assert_eq!(result, 7);

    assert_eq!(nums, result_nums);
}
