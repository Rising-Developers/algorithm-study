function solution(strArr) {
  let sortArr = strArr
    .map((el) => el.length)
    .reduce((acc, cur) => {
      acc[cur] = (acc[cur] || 0) + 1;
      return acc;
    }, {});

  return Math.max(...Object.values(sortArr));
}
