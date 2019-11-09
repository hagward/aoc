import java.io.*;

public class Elves {
    public static int josephus(int n, int k) {
	int[] d = new int[n];
	d[0] = 0;
	d[1] = 0;
	for (int i = 2; i < n; i++) {
	    d[i] = (d[i-1] + k) % n;
	}
	return d[n-1] + 1;
    }

    public static void main(String[] args) {
	int n = (args.length > 0) ? Integer.parseInt(args[0]) : 3018458;
	int k = (args.length > 1) ? Integer.parseInt(args[1]) : 2;
	System.out.printf("n: %d, k: %d%n", n, k);

	for (int i = 2; i < n; i++) {
	    System.out.println(josephus(i, k));
	}
    }
}
