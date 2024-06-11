pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut last: i32 = nums.len() as i32 - 2;
    let mut i: usize = 0;

    if last <= 0 {
        return nums.len() as i32;
    }

    let mut last = last as usize;

    while i < last {
        let mut is_swapped = false;

        println!("{}", i);
        println!("{}", last);

        if i >= last {
            return nums.len() as i32;
        }

        let first = nums[i];
        let second = nums[i + 1];
        let third = nums[i + 2];

        if three_element_is_same(first, second, third) {
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
        swap(nums, index, index + 1);
    }
}

fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
    let temp = nums[i];

    nums[i] = nums[j];
    nums[j] = temp;
}
