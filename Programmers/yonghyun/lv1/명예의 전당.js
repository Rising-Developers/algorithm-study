function solution(k, score) {
  var answer = [];
  for (let i = 0; i < score.length; i++) {
    if (i < k - 1) {
      answer.push(score.slice(0, i + 1).sort((a, b) => b - a)[i]);
    } else {
      answer.push(score.slice(0, i + 1).sort((a, b) => b - a)[k - 1]);
    }
  }
  return answer;
}
