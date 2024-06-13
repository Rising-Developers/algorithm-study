function solution(brown, yellow) {
  // let result = [];

  // let sum = brown + yellow;
  // let measure = [];
  // let idx = 1;
  // while (idx <= sum) {
  //   if (sum % idx === 0) {
  //     measure.push(idx);
  //   }
  //   idx++;
  // }

  // if (measure.length % 2) {
  //   let midNum = Math.floor(measure.length / 2);
  //   return [measure[midNum], measure[midNum]];
  // }

  // for (let i = 0; i < measure.length; i++) {
  //   let boolen =
  //     (measure[i] - 2) * (measure[measure.length - (i + 1)] - 2) === yellow;
  //   if (boolen) {
  //     result.push(measure[i]);
  //   }
  // }

  // return result.reverse();

  let sum = brown + yellow;

  for (let i = 1; i <= Math.sqrt(yellow); i++) {
    if (yellow % i === 0) {
      let width = yellow / i;
      let height = i;
      if ((width + 2) * (height + 2) === sum) {
        return [width + 2, height + 2];
      }
    }
  }
}
