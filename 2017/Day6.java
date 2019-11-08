import java.util.*;

public class Day6 {
    public static void main(String[] args) {
        int[] v = {0, 5, 10, 0, 11, 14, 13, 4, 11, 8, 8, 7, 1, 4, 12, 11};
        Set<Integer> distributions = new HashSet<>();
        Stack<Integer> chain = new Stack<>();

        while (distributions.add(Arrays.hashCode(v))) {
            chain.push(Arrays.hashCode(v));
            redistribute(v, max(v));
        }

        int cycles = 1;
        for (int k = Arrays.hashCode(v); chain.pop() != k; cycles++);

        System.out.println(distributions.size());
        System.out.println(cycles);
    }

    private static void redistribute(int[] v, int i) {
        int k = v[i];
        v[i] = 0;
        for (int j = 1; j <= k; j++) {
            v[(i + j) % v.length]++;
        }
    }

    private static int max(int[] v) {
        int max = 0;
        for (int i = 0; i < v.length; i++) {
            if (v[i] > v[max]) {
                max = i;
            }
        }
        return max;
    }
}
