class Solution {
    int findCenter(int[][] edges) {
       // 첫 번째와 두 번째 엣지에서 노드를 추출합니다
       int node1 = edges[0][0];
       int node2 = edges[0][1];
       int node3 = edges[1][0];
       int node4 = edges[1][1];
       
       // 중심 노드는 첫 번째와 두 번째 엣지 모두에 나타납니다
       if (node1 == node3 || node1 == node4) {
           return node1;
       } else {
           return node2;
       }
   }
}
