// const fnGcd = (n, m) => {
//   let gcd = 1
//   for(let i = 2; i <= Math.min(n, m); i++){
//     if(n % i === 0 && m % i === 0) {
//       gcd = i
//     }
//   }
//   return gcd
// }

// 유클리드 호제법으로 최대공약수 구하기
const fnGcd = (a, b) => (a % b === 0 ? b : fnGcd(b, a % b));

function solution(n, m) {
  let gcd = fnGcd(n, m);
  let lcm = (n / gcd) * (m / gcd) * gcd;

  return [gcd, lcm];
}
