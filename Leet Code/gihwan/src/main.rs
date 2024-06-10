use gihwan::merge_sorted_array;

fn main() {
    let mut arr1 = vec![1, 2, 3, 0, 0, 0];
    let mut arr2 = vec![2, 5, 6];

    merge_sorted_array::solution::merge_with_merge_sort(&mut arr1, 3, &mut arr2, 3);
}
