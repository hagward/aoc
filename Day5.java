import java.io.*;
import java.nio.file.*;

public class Day5 {
    public static void main(String[] args) throws IOException {
        int[] v = Files.lines(Paths.get("Day5Input.txt"))
                .mapToInt(Integer::parseInt)
                .toArray();
        int k = 0;
        for (int i = 0; i >= 0 && i < v.length; i += v[i]++, k++);
        System.out.println(k);
    }
}
