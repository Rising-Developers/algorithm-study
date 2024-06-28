import java.util.HashMap;
import java.util.Map;

public class TwoSum {
    public int[] twoSum(int[] nums, int target) {
        // 값과 인덱스를 저장할 해시맵 생성
        Map<Integer, Integer> numMap = new HashMap<>();
        
        // 배열을 순회하며 필요한 작업 수행
        for (int i = 0; i < nums.length; i++) {
            int num = nums[i];
            // 타겟에서 현재 숫자를 뺀 값(컴플리먼트)을 계산
            int complement = target - num;
            
            // 해시맵에 컴플리먼트가 있는지 확인
            if (numMap.containsKey(complement)) {
                // 있다면 컴플리먼트의 인덱스와 현재 인덱스를 반환
                return new int[] { numMap.get(complement), i };
            }
            
            // 현재 숫자와 그 인덱스를 해시맵에 저장
            numMap.put(num, i);
        }
        
        // 만약 타겟을 만드는 두 수를 찾지 못한 경우 예외 발생
        throw new IllegalArgumentException("No two sum solution");
    }

    // public static void main(String[] args) {
    //     Solution solution = new Solution();
    //     int[] nums = {2, 7, 11, 15};
    //     int target = 9;
    //     int[] result = solution.twoSum(nums, target);
    //     System.out.println("Indices: " + result[0] + ", " + result[1]);
    // }
}