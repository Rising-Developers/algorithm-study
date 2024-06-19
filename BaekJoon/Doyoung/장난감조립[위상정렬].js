const input = require("fs")
  .readFileSync(process.platform === "linux" ? "/dev/stdin" : "./input.txt")
  .toString()
  .trim()
  .split("\n");

const N = parseInt(input.shift());
const M = parseInt(input.shift());

const arr = Array(N + 1).fill(0);
const indegree = Array(N + 1).fill(0);
const DAG = Array.from({ length: N + 1 }, () => new Array(N + 1).fill(0)); // DAG: Directed Acyclic Graph
const isComb = Array(N + 1).fill(false); // isComb: Combination 여부를 저장하는 배열

for (let i = 0; i < M; i++) {
  const [a, b, c] = input[i].split(" ").map(Number);
  DAG[a][b] = c;
  indegree[b]++;
  isComb[a] = true;
}

const queue = [];
for (let i = 1; i <= N; i++) {
  if (indegree[i] === 0) {
    queue.push(i);
    arr[i] = 1;
  }
}

while (queue.length) {
  const node = queue.shift();

  for (let i = 1; i <= N; i++) {
    if (DAG[node][i] !== 0) {
      arr[i] += arr[node] * DAG[node][i];
      indegree[i]--;

      if (indegree[i] === 0) {
        queue.push(i);
      }
    }
  }
}

for (let i = 1; i <= N; i++) {
  if (!isComb[i]) {
    console.log(i, arr[i]);
  }
}
