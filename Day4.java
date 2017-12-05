import java.io.*;
import java.util.*;

public class Day4 {
    public static void main(String[] args) throws IOException {
        int numValid1 = 0;
        int numValid2 = 0;
        try (BufferedReader in = new BufferedReader(new FileReader("Day4Input.txt"))) {
            String line;
            while ((line = in.readLine()) != null) {
                if (validate(line)) {
                    numValid1 += 1;
                }
                if (validate2(line)) {
                    numValid2 += 1;
                }
            }
        }
        System.out.println(numValid1);
        System.out.println(numValid2);
    }

    private static boolean validate(String passphrase) {
        Set<String> words = new HashSet<>();
        for (String word : passphrase.split(" ")) {
            if (!words.add(word)) {
                return false;
            }
        }
        return true;
    }

    private static boolean validate2(String passphrase) {
        Set<Integer> words = new HashSet<>();
        for (String word : passphrase.split(" ")) {
            if (!words.add(hash(word))) {
                return false;
            }
        }
        return true;
    }

    private static int hash(String word) {
        List<Integer> list = new ArrayList<>(26);
        for (int i = 0; i < 26; i++) list.add(0);
        for (char c : word.toCharArray()) {
            int index = c - 97;
            list.set(index, list.get(index) + 1);
        }
        return list.hashCode();
    }
}
