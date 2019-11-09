import java.io.*;
import java.util.*;

public class SignalsAndNoise {

    public void partOne() throws IOException {
        BufferedReader in = new BufferedReader(new FileReader("input.txt"));

        int[][] freq = new int[8][26];
        String line;

        while ((line = in.readLine()) != null) {
            char[] chars = line.toCharArray();
            for (int i = 0; i < chars.length; i++) {
                freq[i][(int) (chars[i] - 'a')]++;
            }
        }

        for (int i = 0; i < freq.length; i++) {
            int maxJ = 0;
            for (int j = 1; j < freq[i].length; j++) {
                if (freq[i][j] > freq[i][maxJ]) {
                    maxJ = j;
                }
            }
            System.out.print((char) (maxJ + 'a'));
        }
        System.out.println();
    }

    public void partTwo() throws IOException {
        BufferedReader in = new BufferedReader(new FileReader("input.txt"));

        int[][] freq = new int[8][26];
        String line;

        while ((line = in.readLine()) != null) {
            char[] chars = line.toCharArray();
            for (int i = 0; i < chars.length; i++) {
                freq[i][(int) (chars[i] - 'a')]++;
            }
        }

        for (int i = 0; i < freq.length; i++) {
            int minJ = 0;
            for (int j = 1; j < freq[i].length; j++) {
                if (freq[i][j] > 0 && freq[i][j] < freq[i][minJ]) {
                    minJ = j;
                }
            }
            System.out.print((char) (minJ + 'a'));
        }
        System.out.println();
    }

    public static void main(String[] args) throws IOException {
        SignalsAndNoise san = new SignalsAndNoise();
        san.partTwo();
    }

}