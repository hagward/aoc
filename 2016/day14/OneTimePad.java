import java.io.*;
import java.util.*;
import java.security.*;
import javax.xml.bind.DatatypeConverter;

public class OneTimePad {

    private final MessageDigest md5;
    private final String salt;
    private final List<String> hashes;

    public OneTimePad(String salt) throws Exception {
        this.md5 = MessageDigest.getInstance("MD5");
        this.salt = salt;
        this.hashes = new ArrayList<>();
    }

    public String getHash(int index) {
        while (hashes.size() <= index) {
            hashes.add(getLowerCaseHash(salt + hashes.size()));
        }
        return hashes.get(index);
    }

    public String getLowerCaseHash(String s) {
	return DatatypeConverter.printHexBinary(md5.digest(s.getBytes())).toLowerCase();
    }

    public String getSuperHash(int index) {
	while (hashes.size() <= index) {
	    String hash = getLowerCaseHash(salt + hashes.size());
	    for (int i = 0; i < 2016; i++) {
		hash = getLowerCaseHash(hash);
	    }
	    hashes.add(hash);
	}
        return hashes.get(index);
    }

    public void partOne(boolean part2) {
        int key = 0;
        int i = 0;
        while (key < 64) {
            String hash = (part2) ? getSuperHash(i) : getHash(i);
            char c = findThreeInARow(hash);
            if (c != '.') {
                for (int j = i+1; j < i+1001; j++) {
                    String hash2 = (part2) ? getSuperHash(j) : getHash(j);
                    if (hasFiveInARow(hash2, c)) {
                        key++;
                        System.out.println("Found key " + key + " at index: " + i + " -> " + j + " | " + hash + " | " + hash2);
                        break;
                    }
                }
            }
            i++;
        }
    }

    public char findThreeInARow(String s) {
        char[] v = s.toCharArray();
        for (int i = 0; i < v.length - 2; i++) {
            if (v[i] == v[i+1] && v[i] == v[i+2]) {
                return v[i];
            }
        }
        return '.';
    }

    public boolean hasFiveInARow(String s, char c) {
        char[] v = s.toCharArray();
        for (int i = 0; i < v.length - 4; i++) {
            if (v[i] == c && v[i+1] == c && v[i+2] == c && v[i+3] == c && v[i+4] == c) {
                return true;
            }
        }
        return false;
    }

    public static void main(String[] args) throws Exception {
        OneTimePad oneTimePad = new OneTimePad(args[0]);
        oneTimePad.partOne(true);

	// System.out.println(oneTimePad.getSuperHash(0));
    }
}
