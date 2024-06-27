const input = require("fs")
  .readFileSync(process.platform === "linux" ? "/dev/stdin" : "./input.txt")
  .toString()
  .trim()
  .split("\n");

const N = parseInt(input.shift());
const map = input.map((row) => row.split(" ").map(Number));
let visited = Array.from(Array(N), () => Array(N).fill(false));
let min = Number.MAX_VALUE;

// 상하좌우
const dx = [0, 0, -1, 1];
const dy = [-1, 1, 0, 0];

function CountIsland(i, j, num) {
  visited[i][j] = true;
  map[i][j] = num;
  for (let k = 0; k < 4; k++) {
    const [newX, newY] = [i + dx[k], j + dy[k]];
    if (
      isValidRange(newX, newY) &&
      !visited[newX][newY] &&
      map[newX][newY] === 1
    ) {
      CountIsland(newX, newY, num);
    }
  }
}

function isValidRange(i, j) {
  return i >= 0 && j >= 0 && i < N && j < N;
}

function connectIsland(islandNumber, check) {
  const queue = [];
  visited = Array.from(Array(N), () => Array(N).fill(false));
  let isBorder = false;

  // 해당 섬 테두리 찾기
  for (let i = 0; i < N; i++) {
    for (let j = 0; j < N; j++) {
      if (!visited[i][j] && map[i][j] === islandNumber) {
        visited[i][j] = true;
        for (let k = 0; k < 4; k++) {
          const [newX, newY] = [i + dx[k], j + dy[k]];
          if (
            isValidRange(newX, newY) &&
            !visited[newX][newY] &&
            map[newX][newY] === 0
          ) {
            isBorder = true;
          }
        }
        if (isBorder) {
          queue.push([i, j, 0]);
          isBorder = false;
        }
      }
    }
  }

  while (queue.length) {
    let [cx, cy, count] = queue.shift();
    for (let i = 0; i < 4; i++) {
      const [newX, newY] = [cx + dx[i], cy + dy[i]];
      if (isValidRange(newX, newY) && !visited[newX][newY]) {
        if (map[newX][newY] === 0) {
          visited[newX][newY] = true;
          queue.push([newX, newY, count + 1]);
        } else if (
          map[newX][newY] !== islandNumber &&
          !check[islandNumber - 1]
        ) {
          min = Math.min(min, count);
        }
      }
    }
  }
}

function solve() {
  // 섬 번호 붙이기
  let islandNumber = 1;
  for (let i = 0; i < N; i++) {
    for (let j = 0; j < N; j++) {
      if (map[i][j] === 1 && !visited[i][j]) {
        CountIsland(i, j, islandNumber);
        islandNumber++;
      }
    }
  }

  //섬 연결하기[BFS]
  let check = Array(islandNumber).fill(false);
  for (let i = 1; i < islandNumber; i++) {
    connectIsland(i, check);
    check[i - 1] = true;
  }

  console.log(min);
}

solve();

// 출처: https://velog.io/@jisubin12/%EB%B0%B1%EC%A4%80-2146-%EB%8B%A4%EB%A6%AC-%EB%A7%8C%EB%93%A4%EA%B8%B0JavaScript
