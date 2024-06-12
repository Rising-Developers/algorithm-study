function solution(n) {
  const binaryOneCnt = (n) => {
    return n.toString(2).match(/1/g).length;
  };

  let originNum = binaryOneCnt(n);

  while (true) {
    n++;
    let nextNum = binaryOneCnt(n);
    if (originNum === nextNum) {
      return n;
      break;
    }
  }
}
