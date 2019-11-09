import java.io.*;
import java.util.*;
import java.util.stream.*;

public class Explosives {

    private int i;
    private String s;
    public int[] times;

    public void partOne() throws IOException {
	String input = new BufferedReader(new FileReader("input.txt"))
		.lines()
		.collect(Collectors.joining());
	System.out.println(expand(input).length());
    }

    public String expand(String s) {
	this.s = s;
	this.i = 0;

	StringBuilder sb = new StringBuilder();
	while (i < s.length()) {
	    char c = s.charAt(i);
	    if (c == '(') {
		i++;
		int a = readInt();
		i++;
		int b = readInt();
		i++;
		String ab = s.substring(i, i + a);
		for (int j = 0; j < b; j++) {
		    sb.append(ab);
		}
		i += a;
	    } else {
		if (c >= 'A' && c <= 'Z') {
		    sb.append(c);
		}
		i++;
	    }
	}
	return sb.toString();
    }

    private int readInt() {
	StringBuilder sb = new StringBuilder();
	char c = s.charAt(i);
	while (Character.isDigit(c)) {
	    sb.append(c);
	    i++;
	    c = s.charAt(i);
	}
	return Integer.parseInt(sb.toString());
    }

    public void partTwo() throws IOException {
	String input = new BufferedReader(new FileReader("input.txt"))
		.lines()
		.collect(Collectors.joining());
	System.out.println(countExpanded(input));
    }

    public long countExpanded(String s) {
	this.s = s;
	this.i = 0;
	this.times = new int[s.length()];
	Arrays.fill(this.times, 1);

	while (i < s.length()) {
	    char c = s.charAt(i);
	    if (c == '(') {
		i++;
		int a = readInt();
		i++;
		int b = readInt();
		i++;
		markTimes(a, b);
	    } else {
		i++;
	    }
	}

	long count = 0;
	for (int j = 0; j < times.length; j++) {
	    char c = s.charAt(j);
	    if (c >= 'A' && c <= 'Z') {
		count += times[j];
	    }
	}
	return count;
    }

    private void markTimes(int a, int b) {
	for (int j = 0; j < a; j++) {
	    times[i+j] *= b;
	}
    }

    public static void main(String[] args) throws IOException {
	Explosives e = new Explosives();
	e.partTwo();
    }
}
