function solution(s) {
  let list = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
  ];
  let new_s = s;
  for (i = 0; i < list.length; i++) {
    if (new_s.includes(list[i])) {
      new_s = new_s.replaceAll(list[i], i);
    }
  }
  return new_s * 1;
}
