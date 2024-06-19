function solution(food) {
  var answer = "";
  for (let i = 1; i < food.length; i++) {
    console.log(food[i] / 2);
    for (let j = 0; j < Math.floor(food[i] / 2); j++) {
      answer = answer + i;
    }
  }
  return answer + "0" + answer.split("").reverse().join("");
}
