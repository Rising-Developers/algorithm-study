const input = require("fs")
  .readFileSync(process.platform === "linux" ? "/dev/stdin" : "./input.txt")
  .toString()
  .trim()
  .split("\n");

const [N, M] = input.shift().split(" ").map(Number);
const board = input.slice(0, N).map((el) => el.split(" ").map(Number));
let dp = Array.from(Array(N + 1), () => Array(N + 1).fill(0));

let answer = "";

// 누적합 배열
for (let i = 1; i < N + 1; i++) {
  for (let j = 1; j < N + 1; j++) {
    dp[i][j] =
      board[i - 1][j - 1] + dp[i][j - 1] + dp[i - 1][j] - dp[i - 1][j - 1];
  }
}

for (let i = N; i < input.length; i++) {
  const [x1, y1, x2, y2] = input[i].split(" ").map(Number);
  // [x2][y2] 값을 구하기위해 x의 끝값과 y의 끝값을 빼준 다음 xy의 끝값을 더해준다.
  answer +=
    String(
      dp[x2][y2] - (dp[x1 - 1][y2] + dp[x2][y1 - 1]) + dp[x1 - 1][y1 - 1]
    ) + "\n";
}

console.log(answer);

// 출처: https://kagrin97-blog.vercel.app/algorithm/11660-%EA%B5%AC%EA%B0%84%20%ED%95%A9%20%EA%B5%AC%ED%95%98%EA%B8%B0%205
