/**
 * @param {Array} arr
 * @param {number} depth
 * @return {Array}
 */
var flat = function (arr, n) {
  let result = [];

  arr.forEach((element) => {
    if (Array.isArray(element) && n > 0) result.push(...flat(element, n - 1));
    else result.push(element);
  });

  return result;
};

// LeetCode 솔루션
var flat = function (arr, n) {
  const ans = [];
  for (var x of arr) {
    if (Array.isArray(x) && n > 0) ans.push(...flat(x, n - 1));
    else ans.push(x);
  }
  return ans;
};
