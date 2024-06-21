function dfs(currentIndex) {
  let sCount = 0;
  for (let i of tree[currentIndex]) {
    sCount += dfs(i);
  }
  if (islandInfo[currentIndex][0] === "W") {
    sCount -= islandInfo[currentIndex][1];
    if (sCount < 0) sCount = 0;
  } else {
    sCount += islandInfo[currentIndex][1];
  }
  return sCount;
}

const input = require("fs")
  .readFileSync(process.platform === "linux" ? "/dev/stdin" : "./input.txt")
  .toString()
  .trim()
  .split("\n");

const N = Number(input.shift());
let board = input.map((row) => row.trim().split(" "));

let islandInfo = [[], [0, 0]];
// const islandInfo = Array(N + 1)
//   .fill(null)
//   .map(() => []); // 섬 정보를 담을 배열
// const islandInfo = Array(N + 1);
let tree = Array.from(Array(N + 1), () => Array().fill([]));

// 양구출작전
// S: 양, W: 늑대

board.forEach((val, idx) => {
  const [t, a, p] = val;
  islandInfo.push([t, +a]);
  tree[+p].push(idx + 2);
});

console.log(dfs(1));

// 출처: https://kagrin97-blog.vercel.app/algorithm/16437-%EC%96%91%20%EA%B5%AC%EC%B6%9C%20%EC%9E%91%EC%A0%84
