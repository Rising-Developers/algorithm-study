const input = require("fs")
  .readFileSync(process.platform === "linux" ? "/dev/stdin" : "../input.txt")
  .toString()
  .split("\n");

let index = 0;
const TC = parseInt(input[index++]);

for (let t = 0; t < TC; t++) {
  const N = parseInt(input[index++]);
  const indegree = Array(N + 1).fill(0);
  const edges = Array.from({ length: N + 1 }, () => Array(N + 1).fill(false));

  const lastYearRanks = input[index++].split(" ").map(Number);
  for (let i = 0; i < N; i++) {
    const num = lastYearRanks[i];
    for (let j = i + 1; j < N; j++) {
      edges[num][lastYearRanks[j]] = true;
      indegree[lastYearRanks[j]]++;
    }
  }

  const m = parseInt(input[index++]);
  for (let i = 0; i < m; i++) {
    const [n1, n2] = input[index++].split(" ").map(Number);
    if (edges[n1][n2]) {
      edges[n1][n2] = false;
      edges[n2][n1] = true;
      indegree[n2]--;
      indegree[n1]++;
    } else {
      edges[n2][n1] = false;
      edges[n1][n2] = true;
      indegree[n1]--;
      indegree[n2]++;
    }
  }

  console.log(topologicalSort(N, indegree, edges));
}

function topologicalSort(N, indegree, edges) {
  const queue = [];
  const result = [];

  for (let i = 1; i <= N; i++) {
    if (indegree[i] === 0) {
      queue.push(i);
    }
  }

  while (queue.length) {
    if (queue.length > 1) return "?";

    const cur = queue.shift();
    result.push(cur);

    for (let i = 1; i <= N; i++) {
      if (edges[cur][i]) {
        if (--indegree[i] === 0) {
          queue.push(i);
        }
      }
    }
  }

  return result.length === N ? result.join(" ") : "IMPOSSIBLE";
}
