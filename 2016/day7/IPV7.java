import java.io.*;
import java.util.*;

public class IPV7 {

    public static void partOne() throws IOException {
        System.out.println(new BufferedReader(new FileReader("input.txt"))
                .lines()
                .filter(ABBA::supportsTls)
                .count());
    }

    public static boolean supportsTls(String s) {
        List<String> parts = extractParts(s);
	for (int i = 1; i < parts.size(); i += 2) {
	    if (hasAbba(parts.get(i))) {
		return false;
	    }
	}
	for (int i = 0; i < parts.size(); i += 2) {
	    if (hasAbba(parts.get(i))) {
		return true;
	    }
	}
	return false;
    }

    public static boolean hasAbba(String s) {
	char[] v = s.toCharArray();
	for (int i = 0; i < v.length - 3; i++) {
	    if (v[i] != v[i+1] && v[i] == v[i+3] && v[i+1] == v[i+2]) {
		return true;
	    }
	}
	return false;
    }

    public static void partTwo() throws IOException {
	System.out.println(new BufferedReader(new FileReader("input.txt"))
		.lines()
		.filter(IPV7::supportsSsl)
		.count());
    }

    public static boolean supportsSsl(String s) {
	List<String> parts = extractParts(s);
	Set<String> abas = new HashSet<>();

	for (int i = 0; i < parts.size(); i += 2) {
	    char[] v = parts.get(i).toCharArray();
	    for (int j = 0; j < v.length - 2; j++) {
		if (v[j] != v[j+1] && v[j] == v[j+2]) {
		    abas.add(String.valueOf(v, j, 3));
		}
	    }
	}

	if (abas.isEmpty()) {
	    return false;
	}

	for (int i = 1; i < parts.size(); i += 2) {
	    char[] v = parts.get(i).toCharArray();
	    for (int j = 0; j < v.length - 2; j++) {
		if (v[j] != v[j+1] && v[j] == v[j+2]
			&& abas.contains(String.join("", Character.toString(v[j+1]), Character.toString(v[j]), Character.toString(v[j+1])))) {
		    return true;
		}
	    }
	}

	return false;
    }

    public static List<String> extractParts(String s) {
	List<String> parts = new ArrayList<>();
	StringBuilder sb = new StringBuilder();

	for (int i = 0; i < s.length(); i++) {
	    char c = s.charAt(i);
	    if (c == '[' || c == ']') {
		parts.add(sb.toString());
		sb = new StringBuilder();
	    } else {
		sb.append(c);
	    }
	}

	parts.add(sb.toString());

	return parts;
    }

    public static void main(String[] args) throws IOException {
	partTwo();
    }

}
