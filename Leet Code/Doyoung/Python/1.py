class Solution(object):
    def twoSum(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[int]
        """
        # Create a dictionary to store the value and its index
        num_map = {}
        
        # Iterate through the list
        for i, num in enumerate(nums): 
            # Calculate the complement that would add up to the target
            complement = target - num
            
            # Check if the complement is in the dictionary
            if complement in num_map:
                # If found, return the current index and the index of the complement
                return [num_map[complement], i]
            
            # Store the current number and its index in the dictionary
            num_map[num] = i
