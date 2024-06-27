function solution(my_string) {
  let arrStr = [...my_string];
  let result = Array.from({ length: 52 }).fill(0);
  let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

  arrStr.forEach((el) => result[alphabet.indexOf(el)]++);

  return result;
}
