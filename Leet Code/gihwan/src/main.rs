fn main() {
    println!("Hello world");

    let mut nums = vec![1];

    let result =
        gihwan::remove_duplicate_from_sorted_array_2::solution::remove_duplicates(&mut nums);

    println!("{:?}", nums);
    println!("{}", result);
}
