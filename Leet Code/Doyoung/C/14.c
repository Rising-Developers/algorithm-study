char* longestCommonPrefix(char** strs, int strsSize) {
    if (strsSize == 0) return "";
    if (strsSize == 1) return strs[0];
    
    int i, j;
    for (i = 0; i < strlen(strs[0]); i++) {
        for (j = 1; j < strsSize; j++) {
            if (strs[0][i] != strs[j][i]) {
                strs[0][i] = '\0';
                return strs[0];
            }
        }
    }
    return strs[0];   
}