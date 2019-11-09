import java.io.*;

public class BathroomSecurity {

    private static final char[][] KEYPAD = {
        { '.', '.', '1', '.', '.' },
        { '.', '2', '3', '4', '.' },
        { '5', '6', '7', '8', '9' },
        { '.', 'A', 'B', 'C', '.' },
        { '.', '.', 'D', '.', '.' }
    };

    private int x;
    private int y;

    public void partOne() throws IOException {
        x = 1;
        y = 1;

        new BufferedReader(new FileReader("input.txt"))
                .lines()
                .map(this::processLinePartOne)
                .forEach(System.out::print);
        System.out.println();
    }

    private int processLinePartOne(String line) {
        for (char c : line.toCharArray()) {
            switch (c) {
                case 'U': y = Math.max(y - 1, 0); break;
                case 'R': x = Math.min(x + 1, 2); break;
                case 'D': y = Math.min(y + 1, 2); break;
                case 'L': x = Math.max(x - 1, 0); break;
            }
        }

        return 3 * y + x + 1;
    }

    public void partTwo() throws IOException {
        x = 0;
        y = 2;

        new BufferedReader(new FileReader("input.txt"))
                .lines()
                .map(this::processLinePartTwo)
                .forEach(System.out::print);
        System.out.println();
    }

    private char processLinePartTwo(String line) {
        for (char c : line.toCharArray()) {
            switch (c) {
                case 'U': if (y > 0 && KEYPAD[y-1][x] != '.') y--; break;
                case 'R': if (x < 4 && KEYPAD[y][x+1] != '.') x++; break;
                case 'D': if (y < 4 && KEYPAD[y+1][x] != '.') y++; break;
                case 'L': if (x > 0 && KEYPAD[y][x-1] != '.') x--; break;
            }
        }

        return KEYPAD[y][x];
    }

    public static void main(String[] args) throws IOException {
        BathroomSecurity bs = new BathroomSecurity();

        if (args[0].equals("p1")) {
            bs.partOne();
        } else if (args[0].equals("p2")) {
            bs.partTwo();
        }
    }

}