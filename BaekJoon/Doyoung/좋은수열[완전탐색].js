const input = require("fs")
  .readFileSync(process.platform === "linux" ? "/dev/stdin" : "./input.txt")
  .toString()
  .trim()
  .split("\n")
  .map((v) => +v)[0];

let end = false;

// let answer = 0;
// const N = input;
// let goodSeq = 0;
// let badSeq = 0;

// function checkSeq(seq) {
//   for (let i = 0; i <= seq.length / 2; i++) {
//     if (seq.slice(-i).join("") === seq.slice(-2 * i, -i).join("")) return false;
//   }
//   return true;
// }

function solve(str) {
  if (end) return;
  if (str.length == input) {
    console.log(str);
    end = true;
    return;
  } else {
    for (let i = 1; i <= 3; i++) {
      const temp = str + `${i}`;
      if (temp.length <= input && isGood(temp)) {
        solve(temp);
      }
    }
  }
}

function isGood(str) {
  const L = str.length;
  const C = Math.floor(L / 2);
  for (let i = 1; i <= C; i++) {
    const A = L;
    const B = L - i;
    const C = L - i * 2;
    if (C >= 0 && str.substring(B, A) == str.substring(C, B)) {
      return false;
    }
  }
  return true;
}

solve("1");

// 출처: https://lhoiktiv.tistory.com/398
