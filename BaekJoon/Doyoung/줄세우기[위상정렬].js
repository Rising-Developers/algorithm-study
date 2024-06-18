const input = require("fs")
  .readFileSync(process.platform === "linux" ? "/dev/stdin" : "./input.txt")
  .toString()
  .trim()
  .split("\n");

// 위상정렬
function makeLine(data) {
  const [[N, _], ...arr] = data.map((el) => el.split(" ").map(Number));

  //   그래프
  // const graph = Array.from({ length: N + 1 }, () => []);
  const graph = Array(N + 1).fill(0);

  //   진입차수
  //   const inDegree = Array(N + 1).fill(null).map(() => []);
  const inDegree = Array(N + 1).fill(0);

  arr.forEach(([a, b]) => {
    // 선행 노드 A 그래프에 b 추가
    graph[a].push(b);

    // 진입차수 증가
    inDegree[b] += 1;
  });

  // 큐 초기화 및 진입차수가 0인 노드 삽입
  const queue = [];
  for (let i = 1; i <= N; i++) {
    if (inDegree[i] === 0) {
      queue.push(i);
    }
  }

  const result = [];
  let queueIdx = 0;

  while (queueIdx < queue.length) {
    const current = queue[queueIdx++];
    result.push(current);

    // 현재 노드와 연결된 노드들의 진입차수 감소
    graph[current].forEach((nextNode) => {
      inDegree[nextNode]--;
      if (inDegree[nextNode] === 0) {
        queue.push(nextNode);
      }
    });
  }

  //   console.log(result.join(" "));
  console.log(...result);
}

const data = input;
makeLine(data);
