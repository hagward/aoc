import java.io.*;
import java.util.*;
import java.util.stream.*;

public class Security {
    public void partOne() throws IOException {
        System.out.println(
                new BufferedReader(new FileReader("input.txt"))
                        .lines()
                        .mapToInt(this::getSectorIdIfValid)
                        .sum());
    }

    private int getSectorIdIfValid(String line) {
        String checksum = line.substring(line.indexOf("[") + 1, line.indexOf("]"));
        if (checksum.equals(calculateChecksum(line))) {
            return getSectorId(line);
        } else {
            return 0;
        }
    }

    private int getSectorId(String line) {
        return Integer.valueOf(line.substring(line.lastIndexOf("-") + 1, line.indexOf("[")));
    }

    private String calculateChecksum(String line) {
        Map<Character, Integer> occurrences = new HashMap<>();
        List<Character> chars = new ArrayList<>();

        for (char c : line.toCharArray()) {
            if (c == '-') continue;
            if (Character.isDigit(c)) break;

            chars.add(c);
            occurrences.put(c, occurrences.getOrDefault(c, 0) + 1);
        }

        return chars
                .stream()
                .distinct()
                .sorted((a, b) -> a - b)
                .sorted((a, b) -> occurrences.get(b) - occurrences.get(a))
                .limit(5)
                .map(Object::toString)
                .collect(Collectors.joining());
    }

    public void partTwo() throws IOException {
        BufferedReader in = new BufferedReader(new FileReader("input.txt"));
        String line;
        while ((line = in.readLine()) != null) {
            String name = decodeName(line);
            if (name.contains("northpole")) {
                System.out.println();
                System.out.println(line);
                System.out.println(name);
            }
        }
    }

    private String decodeName(String line) {
        StringBuilder sb = new StringBuilder();
        for (char c : line.toCharArray()) {
            if (Character.isDigit(c)) break;

            if (c == '-') {
                sb.append(' ');
            } else {
                int i = (((c - 'a') + getSectorId(line)) % 26) + 'a';
                sb.append((char) i);
            }
        }
        return sb.toString();
    }

    public static void main(String[] args) throws IOException {
        Security security = new Security();
        security.partTwo();
    }
}