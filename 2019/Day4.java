public class Day4 {
    public static void main(String[] args) {
        int answer1 = 0;
        int answer2 = 0;

        for (int i = 245182; i < 790572; i++) {
            if (meetsCriteria1(i)) answer1++;
            if (meetsCriteria2(i)) answer2++;
        }

        System.out.println(answer1);
        System.out.println(answer2);
    }

    private static boolean meetsCriteria1(int n) {
        int lastDigit = -1;
        boolean doubleDigits = false;

        for (int i = 5; i >= 0; i--) {
            int divisor = (int) Math.pow(10, i);
            int digit = n / divisor;

            if (lastDigit > -1 && digit < lastDigit) return false;
            if (lastDigit > -1 && digit == lastDigit) doubleDigits = true;

            n -= digit * divisor;
            lastDigit = digit;
        }

        return doubleDigits;
    }

    private static boolean meetsCriteria2(int n) {
        int lastDigit = -1;
        int[] count = new int[10];

        for (int i = 5; i >= 0; i--) {
            int divisor = (int) Math.pow(10, i);
            int digit = n / divisor;

            if (lastDigit > -1 && digit < lastDigit) return false;
            count[digit]++;

            n -= digit * divisor;
            lastDigit = digit;
        }

        for (int c : count) if (c == 2) return true;

        return false;
    }
}
