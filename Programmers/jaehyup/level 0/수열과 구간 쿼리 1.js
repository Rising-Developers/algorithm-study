function solution(arr, queries) {
  queries.forEach((el) => {
    let [a, b] = el;
    for (let i = a; i <= b; i++) {
      arr[i]++;
    }
  });

  return arr;
}
