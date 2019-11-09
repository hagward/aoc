import java.io.*;

public class Timing {

    public static void solvePartOne() {
	int[] a = { 5, 8, 1, 7, 1, 0 };
	int[] n = { 17, 19, 7, 13, 5, 3 };
	solve(a, n);
    }

    public static void solvePartTwo() {
	int[] a = { 5, 8, 1, 7, 1, 0, 0 };
	int[] n = { 17, 19, 7, 13, 5, 3, 11 };
	solve(a, n);
    }

    public static void solve(int[] a, int[] n) {
	int i = 0;

	outer: while (true) {
	    int x = 11 + 17 * i++;
	    // System.out.println("x: " + x);
	    for (int j = 1; j < a.length; j++) {
		int x1 = (a[j] + x) % n[j];
		int n1 = (n[j] - j - 1) % n[j];
		// if (j > 2) {
		//     System.out.printf("  %d: %d != %d%n", j, x1, n1);
		// }
		if (x1 != n1) {
		    continue outer;
		}
	    }
	    System.out.printf("i: %d, x: %d%n", i, x);
	    return;
	}
    }

    public static void main(String[] args) {
	solvePartTwo();
    }
}
