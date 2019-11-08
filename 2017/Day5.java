import java.io.*;
import java.nio.file.*;
import java.util.*;

public class Day5 {
    public static void main(String[] args) throws IOException {
        int[] v1 = Files.lines(Paths.get("Day5Input.txt"))
                .mapToInt(Integer::parseInt)
                .toArray();
        int[] v2 = Arrays.copyOf(v1, v1.length);
        System.out.println(partOne(v1));
        System.out.println(partTwo(v2));
    }

    private static int partOne(int[] v) {
        int k = 0;
        for (int i = 0; i >= 0 && i < v.length; i += v[i]++, k++);
        return k;
    }

    private static int partTwo(int[] v) {
        int k = 0;
        for (int i = 0; i >= 0 && i < v.length; k++) {
            int j = i;
            i += v[i];
            v[j] += (v[j] >= 3) ? -1 : 1;
        }
        return k;
    }
}
