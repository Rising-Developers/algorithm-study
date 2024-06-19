function solution(sizes) {
  let maxRow = 0;
  let maxCol = 0;

  // sizes.forEach((el) => {
  //   const [row, col] = el.sort((a, b) => b - a)
  //   maxRow = Math.max(maxRow, row)
  //   maxCol = Math.max(maxCol, col)
  // })

  sizes.forEach((el) => {
    maxRow = Math.max(maxRow, Math.max(...el));
    maxCol = Math.max(maxCol, Math.min(...el));
  });

  return maxRow * maxCol;
}
