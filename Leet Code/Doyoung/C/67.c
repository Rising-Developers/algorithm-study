char* addBinary(char* a, char* b) {
    int a_len = strlen(a);
    int b_len = strlen(b);
    int max_len = a_len > b_len ? a_len : b_len;
    int carry = 0;
    char* result = (char*)malloc(sizeof(char) * (max_len + 2));
    result[max_len + 1] = '\0';
    for (int i = 0; i < max_len; i++) {
        int a_val = a_len - i - 1 >= 0 ? a[a_len - i - 1] - '0' : 0;
        int b_val = b_len - i - 1 >= 0 ? b[b_len - i - 1] - '0' : 0;
        int sum = a_val + b_val + carry;
        carry = sum / 2;
        result[max_len - i] = sum % 2 + '0';
    }
    result[0] = carry + '0';
    if (result[0] == '0') {
        return result + 1;
    }
    return result;
}