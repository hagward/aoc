import java.io.*;
import java.util.*;
import java.security.*;
import javax.xml.bind.DatatypeConverter;

public class TwoStepsForward {

    private final String input;
    private final MessageDigest md5;

    public TwoStepsForward(String input) throws Exception {
	this.input = input;
	this.md5 = MessageDigest.getInstance("MD5");
    }

    public String md5(String s) {
	return DatatypeConverter.printHexBinary(md5.digest(s.getBytes())).substring(0, 4);
    }

    public String findShortestPath() {
	Queue<String> queue = new LinkedList<>();

	queue.add("");

	while (!queue.isEmpty()) {
	    String path = queue.remove();
	    int[] p = position(path);

	    if (p[0] == 3 && p[1] == 3) {
		return path;
	    }

	    String hash = md5(input + path);
	    // System.out.printf("%d,%d: %s - %s", p[0], p[1], path, hash);

	    if (p[1] > 0 && isOpen(hash.charAt(0))) {
		queue.add(path + "U");
		// System.out.print(" U");
	    }
	    if (p[1] < 3 && isOpen(hash.charAt(1))) {
		queue.add(path + "D");
		// System.out.print(" D");
	    }
	    if (p[0] > 0 && isOpen(hash.charAt(2))) {
		queue.add(path + "L");
		// System.out.print(" L");
	    }
	    if (p[0] < 3 && isOpen(hash.charAt(3))) {
		queue.add(path + "R");
		// System.out.print(" R");
	    }

	    // System.out.println();
	}

	return "";
    }

    public int findLongestPathLength() {
	int maxLength = 0;

	Queue<String> queue = new LinkedList<>();
	queue.add("");

	while (!queue.isEmpty()) {
	    String path = queue.remove();
	    int[] p = position(path);

	    if (p[0] == 3 && p[1] == 3) {
		if (path.length() > maxLength) {
		    maxLength = path.length();
		}
		continue;
	    }

	    String hash = md5(input + path);
	    // System.out.printf("%d,%d: %s - %s", p[0], p[1], path, hash);

	    if (p[1] > 0 && isOpen(hash.charAt(0))) {
		queue.add(path + "U");
		// System.out.print(" U");
	    }
	    if (p[1] < 3 && isOpen(hash.charAt(1))) {
		queue.add(path + "D");
		// System.out.print(" D");
	    }
	    if (p[0] > 0 && isOpen(hash.charAt(2))) {
		queue.add(path + "L");
		// System.out.print(" L");
	    }
	    if (p[0] < 3 && isOpen(hash.charAt(3))) {
		queue.add(path + "R");
		// System.out.print(" R");
	    }

	    // System.out.println();
	}

	return maxLength;
    }

    public boolean isOpen(char c) {
	return c >= 'B' && c <= 'F';
    }

    public int[] position(String s) {
	int x = 0;
	int y = 0;

	for (int i = 0; i < s.length(); i++) {
	    switch (s.charAt(i)) {
		case 'U': y--; break;
		case 'R': x++; break;
		case 'D': y++; break;
		case 'L': x--; break;
	    }
	}

	return new int[]{x, y};
    }

    public static void main(String[] args) throws Exception {
	TwoStepsForward t = new TwoStepsForward(args[0]);
	System.out.println(t.findLongestPathLength());
    }
}
