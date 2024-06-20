const input = require("fs")
  .readFileSync(process.platform === "linux" ? "/dev/stdin" : "./input.txt")
  .toString()
  .trim()
  .split("\n");

const [N, M] = input.shift().split(" ").map(Number);
const map = input.map((v) => v.split(" ").map(Number));
const visited = Array.from({ length: N + 1 }, () =>
  Array.from({ length: M + 1 }, () => 1)
);

// 상하좌우
const dx = [0, 0, 1, -1];
const dy = [1, -1, 0, 0];

let answer = 0;

// 0: 내부공기, 1: 치즈, 2: 외부공기, 3: 녹은상태

const checkInOut = () => {
  const queue = [];
  queue.push([0, 0]);
  visited.map((v) => v.fill(0));

  while (queue.length) {
    const [x, y] = queue.shift();

    for (let i = 0; i < 4; i++) {
      const [nx, ny] = [x + dx[i], y + dy[i]];

      if (nx >= 0 && nx < N && ny >= 0 && ny < M) {
        if (!visited[nx][ny] && map[nx][ny] !== 1) {
          visited[nx][ny] = 1;
          map[nx][ny] = 2;
          queue.push([nx, ny]);
        }
      }
    }
  }
};

let arrHasCheese = true;

while (arrHasCheese) {
  let isMelt = false;
  checkInOut();

  for (let i = 0; i < N; i++) {
    for (let j = 0; j < M; j++) {
      if (map[i][j] === 1) {
        let cnt = 0;
        for (let k = 0; k < 4; k++) {
          const [nx, ny] = [i + dx[k], j + dy[k]];
          if (nx >= 0 && nx < N && ny >= 0 && ny < M && map[nx][ny] === 2)
            cnt++;
        }
        if (cnt >= 2) {
          map[i][j] = 3;
          isMelt = true;
        }
      }
    }
  }
  if (isMelt) {
    map.forEach((row) =>
      row.forEach((v) => {
        if (v === 3) v = 2;
      })
    );
  }
  answer++;

  arrHasCheese = false;

  map.forEach((row) =>
    row.forEach((v) => {
      if (v === 1) arrHasCheese = true;
    })
  );

  if (!arrHasCheese) break;
}

console.log(answer);

// 출처: https://velog.io/@ywc8851/%EB%B0%B1%EC%A4%80-2638-%EC%B9%98%EC%A6%88-javascript
