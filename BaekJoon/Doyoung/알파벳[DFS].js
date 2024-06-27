const input = require("fs")
  .readFileSync(process.platform === "linux" ? "/dev/stdin" : "./input.txt")
  .toString()
  .trim()
  .split("\n");

const [R, C] = input.shift().split(" ").map(Number);
let board = input.map((row) => row.trim().split(""));

let visited = Array(26).fill(false);
let answer = 0;

// 상하좌우
const dx = [0, 0, -1, 1];
const dy = [-1, 1, 0, 0];

function dfs(x, y, cnt) {
  answer = Math.max(answer, cnt);
  for (let i = 0; i < 4; i++) {
    const [newX, newY] = [x + dx[i], y + dy[i]];
    if (
      newX >= 0 &&
      newX < R &&
      newY >= 0 &&
      newY < C &&
      !visited[board[newX][newY].charCodeAt(0) - 65]
    ) {
      visited[board[newX][newY].charCodeAt(0) - 65] = true;
      dfs(newX, newY, cnt + 1);
      visited[board[newX][newY].charCodeAt(0) - 65] = false;
    }
  }
}

visited[board[0][0].charCodeAt() - 65] = true;
dfs(0, 0, 1);
console.log(answer);

// 출처: https://velog.io/@ywc8851/%EB%B0%B1%EC%A4%80-1987-%EC%95%8C%ED%8C%8C%EB%B2%B3-javascript
