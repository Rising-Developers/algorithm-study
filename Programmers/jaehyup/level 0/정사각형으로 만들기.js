function solution(arr) {
  let col = arr.length;
  let row = arr[0].length;

  if (col > row) {
    for (let i = 0; i < arr.length; i++) {
      for (let k = row; k < col; k++) {
        arr[i].push(0);
      }
    }
  } else if (col < row) {
    for (let i = col; i < row; i++) {
      let newArr = Array.from({ length: row }).fill(0);
      arr.push(newArr);
    }
  }

  return arr;
}
