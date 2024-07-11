/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* plusOne(int* digits, int digitsSize, int* returnSize) {
    int i = digitsSize - 1;
    int carry = 1;
    while (i >= 0 && carry) {
        digits[i] += carry;
        carry = digits[i] / 10;
        digits[i] %= 10;
        i--;
    }
    if (carry) {
        *returnSize = digitsSize + 1;
        int* result = (int*)malloc(sizeof(int) * (*returnSize));
        result[0] = 1;
        for (int i = 0; i < digitsSize; i++) {
            result[i + 1] = digits[i];
        }
        return result;
    }
    *returnSize = digitsSize;
    return digits;
}