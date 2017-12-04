import java.io.*;
import java.util.*;

public class Day4 {
    public static void main(String[] args) throws IOException {
        int numValid = 0;
        try (BufferedReader in = new BufferedReader(new FileReader("Day4Input.txt"))) {
            String line;
            while ((line = in.readLine()) != null) {
                if (validate(line)) {
                    numValid += 1;
                }
            }
        }
        System.out.println(numValid);
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
}
