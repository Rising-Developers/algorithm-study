function solution(A, B) {
  let sortA = A.sort((a, b) => a - b);
  let sortB = B.sort((a, b) => b - a);
  return sortA.reduce((acc, cur, idx) => acc + cur * sortB[idx], 0);
}
