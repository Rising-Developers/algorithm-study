const input = require("fs")
  .readFileSync(process.platform === "linux" ? "/dev/stdin" : "./input.txt")
  .toString()
  .trim()
  .split("\n");

const N = Number(input.shift());
const map = input.map((e) => e.split(" ").map(Number));
let answer = 0;

// 상하좌우
const dx = [-1, 1, 0, 0];
const dy = [0, 0, -1, 1];

function isSafe(x, y, rainHeight) {
  return map[x][y] > rainHeight;
}

function DFS(x, y, rainHeight, visited) {
  visited[x][y] = true;

  for (let i = 0; i < 4; i++) {
    const [nx, ny] = [x + dx[i], y + dy[i]];

    if (
      nx >= 0 &&
      nx < N &&
      ny >= 0 &&
      ny < N &&
      !visited[nx][ny] &&
      isSafe(nx, ny, rainHeight)
    ) {
      DFS(nx, ny, rainHeight, visited);
    }
  }
}

const maxHeight = Math.max(...map.flat());

for (let rainHeight = 0; rainHeight < maxHeight; rainHeight++) {
  let visited = Array.from(Array(N), () => Array(N).fill(false));
  let safeZones = 0;

  for (let i = 0; i < N; i++) {
    for (let j = 0; j < N; j++) {
      if (!visited[i][j] && isSafe(i, j, rainHeight)) {
        DFS(i, j, rainHeight, visited);
        safeZones++;
      }
    }
  }

  answer = Math.max(answer, safeZones);
}

console.log(answer);
