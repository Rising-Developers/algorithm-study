function solution(arr) {
  let idx = arr.length;
  while (idx) {
    if (Number.isInteger(Math.log2(idx))) {
      break;
    }
    idx++;
    arr.push(0);
  }
  return arr;
}
