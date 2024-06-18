const fnGcd = (a, b) => (a % b === 0 ? b : fnGcd(b, a % b));
const fnLcm = (a, b) => (a * b) / fnGcd(a, b);

function solution(arr) {
  let result = arr[0];

  for (let i = 1; i < arr.length; i++) {
    result = fnLcm(result, arr[i]);
  }

  return result;
}
