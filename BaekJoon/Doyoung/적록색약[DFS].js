const input = require("fs")
  .readFileSync(process.platform === "linux" ? "/dev/stdin" : "./input.txt")
  .toString()
  .trim()
  .split("\n");

const N = Number(input.shift());
// const map = input.map((e) => e.split("").map(Number));
const map = input.map((e) => e.split(""));
let visited = Array.from(Array(N), () => Array(N).fill(false));

let rgb = 0;
let gb = 0;

// 상하좌우
const dx = [0, 0, 1, -1];
const dy = [1, -1, 0, 0];

function DFS(x, y) {
  visited[x][y] = true;

  for (let i = 0; i < 4; i++) {
    const [nx, ny] = [x + dx[i], y + dy[i]];

    if (nx < 0 || nx >= N || ny < 0 || ny >= N || visited[nx][ny]) continue;

    if (map[nx][ny] == map[x][y]) DFS(nx, ny);
  }
}

// 적록색약이 아닌 사람
for (let i = 0; i < N; i++) {
  for (let j = 0; j < N; j++) {
    if (!visited[i][j]) {
      DFS(i, j);
      rgb++;
    }
  }
}

// 방문기록 초기화
visited = Array.from(Array(N), () => Array(N).fill(false));

// R을 G로 바꿔줌
for (let i = 0; i < N; i++) {
  for (let j = 0; j < N; j++) {
    if (map[i][j] === "R") map[i][j] = "G";
  }
}

// 적록색약인 사람
for (let i = 0; i < N; i++) {
  for (let j = 0; j < N; j++) {
    if (!visited[i][j]) {
      DFS(i, j);
      gb++;
    }
  }
}

console.log(rgb + " " + gb);

// 출처: https://velog.io/@jiyaho/%EB%B0%B1%EC%A4%80-10026-%EC%A0%81%EB%A1%9D%EC%83%89%EC%95%BD-Node.js-DFS-BFS-%ED%92%80%EC%9D%B4
