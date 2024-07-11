int lengthOfLastWord(char* s) {
    int len = strlen(s);
    int count = 0;
    for (int i = len - 1; i >= 0; i--) {
        if (s[i] == ' ') {
            if (count == 0) {
                continue;
            }
            break;
        }
        count++;
    }
    return count;
}