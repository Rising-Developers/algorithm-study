function solution(left, right) {
  let result = 0;

  //   const measure = (num) => {
  //     let result = [];
  //     let idx = 1;
  //     while (idx <= num) {
  //       if (num % idx === 0) {
  //         result.push(idx);
  //       }
  //       idx++;
  //     }
  //     return result.length;
  //   };

  //   for (let i = left; i <= right; i++) {
  //     let num = measure(i);
  //     num % 2 ? (result -= i) : (result += i);
  //   }

  for (let i = left; i <= right; i++) {
    // 제곱근이 정수면 약수의 개수가 홀수다
    Number.isInteger(Math.sqrt(i)) ? (result -= i) : (result += i);
  }

  return result;
}
