function solution(num) {
  let count = 0;
  
  while (num !== 1) { // num이 1이 아닌 동안 계속 반복
      num = num % 2 === 0 ? num / 2 : (num * 3)+1
      count++;
      if(count === 500){
          return -1;
      }
  }
  return count;
}