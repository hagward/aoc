import java.io.*;
import java.nio.file.*;
import java.util.*;
import java.util.function.*;
import java.util.stream.*;

public class Day4 {
    public static void main(String[] args) throws IOException {
        List<String> lines = Files.lines(Paths.get("Day4Input.txt"))
                .collect(Collectors.toList());

        long partOne = lines.stream()
                .filter(line -> validate(line, String::hashCode))
                .count();

        long partTwo = lines.stream()
                .filter(line -> validate(line, Day4::anagramHash))
                .count();

        System.out.println(partOne);
        System.out.println(partTwo);
    }

    private static boolean validate(String passphrase, Function<String, Integer> hashFunction) {
        Set<Integer> hashes = new HashSet<>();
        for (String word : passphrase.split(" ")) {
            if (!hashes.add(hashFunction.apply(word))) {
                return false;
            }
        }
        return true;
    }

    private static int anagramHash(String word) {
        List<Integer> list = new ArrayList<>(26);
        for (int i = 0; i < 26; i++) list.add(0);
        for (char c : word.toCharArray()) {
            int index = c - 97;
            list.set(index, list.get(index) + 1);
        }
        return list.hashCode();
    }
}
