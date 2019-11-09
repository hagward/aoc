import java.io.*;
import java.util.*;

public class DragonChecksum {

    public static String fill(String initial, int length) {
	int n = initial.length();
	String s = initial;
	while (n < length) {
	    s = expand(s);
	    n = s.length();
	}
	return s.substring(0, length);
    }

    public static String expand(String a) {
	StringBuilder b = new StringBuilder(a).reverse();
	for (int i = 0; i < b.length(); i++) {
	    b.setCharAt(i, (b.charAt(i) == '0') ? '1' : '0');
	}
	return new StringBuilder().append(a).append('0').append(b).toString();
    }

    public static String checksum(String s) {
	char[] oldV = s.toCharArray();
	while (true) {
	    char[] newV = new char[oldV.length / 2];
	    for (int i = 0, j = 0; i < oldV.length; i += 2, j++) {
		newV[j] = (oldV[i] == oldV[i+1]) ? '1' : '0';
	    }
	    if (newV.length % 2 == 1) {
		return String.valueOf(newV);
	    }
	    oldV = newV;
	}
    }

    public static void main(String[] args) {
	System.out.println(checksum(fill(args[0], Integer.parseInt(args[1]))));
	// System.out.println(expand(args[0]));
	// System.out.println(checksum(args[0]));
    }

}
