class Solution(object):
    def findCenter(self, edges):
        # 첫 번째와 두 번째 엣지에서 노드를 추출합니다
        node1 = edges[0][0]
        node2 = edges[0][1]
        node3 = edges[1][0]
        node4 = edges[1][1]
        
        # 중심 노드는 첫 번째와 두 번째 엣지 모두에 나타납니다
        if node1 == node3 or node1 == node4:
            return node1
        else:
            return node2

# 예제 1
edges1 = [[1, 2], [2, 3], [4, 2]]
solution = Solution()
center1 = solution.findCenter(edges1)
print(f"Center of the star graph (Example 1): {center1}") # 출력: 2

# 예제 2
edges2 = [[1, 2], [5, 1], [1, 3], [1, 4]]
center2 = solution.findCenter(edges2)
print(f"Center of the star graph (Example 2): {center2}") # 출력: 1