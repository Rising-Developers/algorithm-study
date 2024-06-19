function solution(strings, n) {
  var answer = [];
  let pick_list = [];
  let new_strings = strings.sort();
  for (let i = 0; i < new_strings.length; i++) {
    pick_list.push({ [i]: new_strings[i][n] });
  }
  pick_list.sort((a, b) => {
    let valueA = Object.values(a)[0];
    let valueB = Object.values(b)[0];

    if (valueA < valueB) {
      return -1;
    }
    if (valueA > valueB) {
      return 1;
    }
    return 0;
  });
  answer = pick_list.map((ele) => new_strings[Object.keys(ele)[0] * 1]);
  return answer;
}
