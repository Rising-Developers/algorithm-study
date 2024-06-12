function solution(a, b, n) {
  var answer = 0;
  let coke = n;
  while (coke >= a) {
    answer = answer + Math.floor(coke / a) * b;
    coke = (coke % a) + Math.floor(coke / a) * b;
    console.log(coke);
  }
  return answer;
}
