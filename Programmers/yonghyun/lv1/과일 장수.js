function solution(k, m, score) {
  var answer = 0;

  function chunkArray(array, size) {
    const result = [];
    for (let i = 0; i < array.length; i += size) {
      const chunk = array.slice(i, i + size);
      result.push(chunk);
    }
    return result;
  }

  let newArr = chunkArray(
    score.sort((a, b) => b - a),
    m
  );
  for (let i = 0; i < newArr.length; i++) {
    if (newArr[i].length == m) {
      answer += newArr[i][m - 1] * m;
    }
  }
  return answer;
}
