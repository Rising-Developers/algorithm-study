public class PalindromeNumber {
    public boolean isPalindrome(int x) {

        if (x < 0) {
            return false;
        }

        String strX = Integer.toString(x);

        String reversedStrX = new StringBuilder(strX).reverse().toString();
        return strX.equals(reversedStrX);
    }

}
