class Solution {
    public int strStr(String haystack, String needle) {
        if (needle.length() == 0) return 0;
        int index = -1;
        for (int i = 0; i < haystack.length(); i++) {
            if (haystack.charAt(i) == needle.charAt(0)) {
                if (haystack.length() - i < needle.length()) break;
                boolean isSame = true;
                for (int j = 1; j < needle.length(); j++) {
                    if (haystack.charAt(i + j) != needle.charAt(j)) {
                        isSame = false;
                        break;
                    }
                }
                if (isSame) {
                    index = i;
                    break;
                }
            }
        }
        return index;
    }
}