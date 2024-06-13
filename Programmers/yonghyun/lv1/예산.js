function solution(d, budget) {
  var answer = 0;
  let new_d = d.sort((a, b) => a - b);
  let new_budget = budget;
  for (let i = 0; i < new_d.length; i++) {
    new_budget = new_budget - new_d[i];
    if (new_budget < 0) {
      return answer;
    } else {
      if (i == new_d.length - 1) {
        return ++answer;
      }
      answer++;
    }
  }
}
