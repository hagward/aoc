import java.io.*;
import java.nio.file.*;

public class Day9 {

    private char[] v;
    private int i;
    private int garbage;

    public Day9(String input) {
        this.v = input.toCharArray();
        this.i = 0;
        this.garbage = 0;
    }

    private int score(int level) {
        int score = level;

        while (i < v.length && v[i] != '}') {
            char chr = v[i++];
            if (chr == '{') {
                score += score(level + 1);
            } else if (chr == '<') {
                skip();
            } else if (chr == '!') {
                i++;
            }
        }

        i++;

        return score;
    }

    private void skip() {
        while (i < v.length && v[i] != '>') {
            if (v[i] == '!') {
                i++;
            } else {
                garbage++;
            }
            i++;
        }
    }

    public static void main(String[] args) throws IOException {
        String input = new String(Files.readAllBytes(Paths.get("Day9Input.txt")));
        Day9 day9 = new Day9(input);
        System.out.println(day9.score(0));
        System.out.println(day9.garbage);
    }
}
