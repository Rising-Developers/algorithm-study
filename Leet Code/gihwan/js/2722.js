/**
 * @param {Array>} arr1
 * @param {Array} arr2
 * @return {Array}
 */
var join = function (arr1, arr2) {
  const map = new Map();

  arr1.forEach((value) => {
    map.set(value.id, value);
  });

  arr2.forEach((value) => {
    const existingValue = map.get(value.id);

    if (existingValue) {
      const newValue = { ...existingValue, ...value };
      map.set(value.id, newValue);
    } else {
      map.set(value.id, value);
    }
  });

  return [...map.values()].sort((a, b) => a.id - b.id);
};

var leetCodeSolution = function (arr1, arr2) {
  let map = {};
  arr1.forEach((obj) => (map[obj.id] = obj));
  arr2.forEach((obj) => {
    if (map[obj.id]) {
      map[obj.id] = { ...map[obj.id], ...obj };
    } else {
      map[obj.id] = obj;
    }
  });
  return [...Object.values(map)];
};
