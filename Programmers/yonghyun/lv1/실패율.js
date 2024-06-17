function solution(N, stages) {
  var answer = [];
  for (let i = 0; i < N; i++) {
    let Y = 0;
    let N = 0;
    for (let j = 0; j < stages.length; j++) {
      if (stages[j] > i + 1) {
        Y++;
      } else if (stages[j] == i + 1) {
        N++;
      }
    }
    answer.push(N / Y);
  }

  const indices = answer.map((value, index) => index).map((val) => val + 1);
  indices.sort((a, b) => answer[b - 1] - answer[a - 1]);
  return indices;
}
