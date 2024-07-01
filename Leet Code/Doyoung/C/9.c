#include <stdio.h>
#include <stdbool.h>
#include <string.h>

bool isPalindrome(int x) {
    
    if (x < 0) {
        return false;
    }

    char str[12]; // 32비트 정수의 최대 길이는 10자리 + 음수부호 + NULL 문자
    sprintf(str, "%d", x);

    // 문자열을 뒤집습니다.
    int len = strlen(str);
    for (int i = 0; i < len / 2; i++) {
        if (str[i] != str[len - 1 - i]) {
            return false;
        }
    }
    return true;
}

