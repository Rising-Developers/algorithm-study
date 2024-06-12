pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = prices[0];
    let mut total = 0;

    for price in prices[1..].iter() {
        if *price < min_price {
            min_price = *price;
        } else if *price > min_price {
            total += *price - min_price;
            min_price = *price;
        }
    }

    total
}

#[test]
fn test_case_1() {
    let prices = vec![7, 1, 5, 3, 6, 4];

    assert_eq!(max_profit(prices), 7);
}

#[test]
fn test_case_2() {
    let prices = vec![1, 2, 3, 4, 5];

    assert_eq!(max_profit(prices), 4);
}

#[test]
fn test_case_3() {
    let prices = vec![7, 6, 4, 3, 1];

    assert_eq!(max_profit(prices), 0);
}
