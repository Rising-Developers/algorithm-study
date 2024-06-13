function solution(a, b, n) {
  let service = 0;

  while (a <= n) {
    let extra = Math.floor(n / a) * b;
    service += extra;
    n = Math.floor(n / a) * b + (n % a);
  }

  return service;
}
