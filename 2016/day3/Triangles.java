import java.io.*;
import java.util.*;

public class Triangles {

    public void partOne() throws IOException {
        long possibleTriangles = new BufferedReader(new FileReader("input.txt"))
                .lines()
                .filter(this::isPossibleTriangle)
                .count();
        System.out.println(possibleTriangles);
    }

    public void partTwo() throws IOException {
        BufferedReader in = new BufferedReader(new FileReader("input.txt"));

        int possibleTriangles = 0;

        int[] x = new int[3];
        int[] y = new int[3];
        int[] z = new int[3];
        outer: while (true) {
            for (int i = 0; i < 3; i++) {
                String line = in.readLine();

                if (line == null) {
                    break outer;
                }

                String[] numbers = line.trim().split("\\s+");
                x[i] = Integer.parseInt(numbers[0]);
                y[i] = Integer.parseInt(numbers[1]);
                z[i] = Integer.parseInt(numbers[2]);
            }

            if (isPossibleTriangle(x[0], x[1], x[2])) possibleTriangles++;
            if (isPossibleTriangle(y[0], y[1], y[2])) possibleTriangles++;
            if (isPossibleTriangle(z[0], z[1], z[2])) possibleTriangles++;
        }

        System.out.println(possibleTriangles);
    }

    private boolean isPossibleTriangle(String line) {
        String[] numbers = line.trim().split("\\s+");
        return isPossibleTriangle(
                Integer.parseInt(numbers[0]),
                Integer.parseInt(numbers[1]),
                Integer.parseInt(numbers[2]));
    }

    private boolean isPossibleTriangle(int a, int b, int c) {
        return a + b > c && a + c > b && b + c > a;
    }

    public static void main(String[] args) throws IOException {
        Triangles t = new Triangles();

        if (args[0].equals("p1")) {
            t.partOne();
        } else if (args[0].equals("p2")) {
            t.partTwo();
        }
    }

}