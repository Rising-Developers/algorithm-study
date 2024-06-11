fn main() {
    println!("Hello world");

    let num = vec![2, 2, 1, 1, 1, 2, 2];

    let result = gihwan::majority_element::solution::majority_element(num);

    println!("{}", result);
}
