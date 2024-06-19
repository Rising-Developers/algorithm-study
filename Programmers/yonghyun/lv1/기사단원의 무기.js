function solution(number, limit, power) {
  var answer = 0;

  function countDivisors(n) {
    let divisorsCount = new Array(n + 1).fill(0);

    for (let i = 1; i <= n; i++) {
      for (let j = i; j <= n; j += i) {
        divisorsCount[j]++;
      }
    }

    return divisorsCount;
  }

  let newArr = countDivisors(number);

  for (let i = 0; i < newArr.length; i++) {
    if (newArr[i] > limit) {
      answer += power * 1;
    } else {
      answer += newArr[i] * 1;
    }
  }
  return answer;
}
