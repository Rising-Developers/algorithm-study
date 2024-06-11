function solution(sizes) {
  let maxWidth = 0;
  let maxHeight = 0;
  
  for (const [width, height] of sizes) {
      maxWidth = Math.max(maxWidth, Math.max(width, height));
      maxHeight = Math.max(maxHeight, Math.min(width, height));
  }
  
  return maxWidth * maxHeight;
}

/*
1. 단순 브루트포스(반복문) 으로 문제풀기
*/ 