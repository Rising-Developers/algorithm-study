function solution(s) {
  let idx = 0;

  return [...s]
    .map((el) => {
      el === " " ? (idx = 0) : idx++;
      return (idx - 1) % 2 ? el.toLowerCase() : el.toUpperCase();
    })
    .join("");
}
