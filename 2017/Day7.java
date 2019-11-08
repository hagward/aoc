import java.io.*;
import java.nio.file.*;
import java.util.*;
import java.util.stream.*;

public class Day7 {
    public static void main(String[] args) throws IOException {
        List<String> lines = Files.lines(Paths.get("Day7Input.txt"))
                .collect(Collectors.toList());
        partOne(lines);
    }

    private static void partOne(List<String> lines) {
        Set<String> rootDiscs = new HashSet<>();
        Set<String> childDiscs = new HashSet<>();
        for (String line : lines) {
            String[] parts = line.split(" -> ");
            rootDiscs.add(parts[0].split(" ")[0]);
            if (parts.length == 2) {
                for (String childDisc : parts[1].split(", ")) {
                    childDiscs.add(childDisc);
                }
            }
        }
        rootDiscs.removeAll(childDiscs);
        System.out.println(rootDiscs);
    }
}
