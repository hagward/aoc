import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;

public class Day5 {
    public static void main(String[] args) throws IOException {
        System.out.println("Running with input = 1...");
        run(getInput(), 1);

        System.out.println("Running with input = 5...");
        run(getInput(), 5);
    }

    private static int[] getInput() throws IOException {
        String[] split = new String(Files.readAllBytes(Paths.get("inputs/day5.txt")), "UTF-8").split(",");
        int[] result = new int[split.length];
        for (int i = 0; i < split.length; i++) {
            result[i] = Integer.parseInt(split[i].trim());
        }
        return result;
    }

    private static void run(int[] input, int in) {
        int i = 0;
        while (i < input.length) {
            char[] op = Integer.toString(input[i]).toCharArray();

            if (op.length > 1 && op[op.length-1] == '9' && op[op.length-2] == '9') return;

            switch (op[op.length-1]) {
                case '1': {
                    int a = op.length > 2 && op[op.length-3] == '1' ? input[i+1] : input[input[i+1]];
                    int b = op.length > 3 && op[op.length-4] == '1' ? input[i+2] : input[input[i+2]];
                    input[input[i+3]] = a + b;
                    i += 4;
                    break;
                }
                case '2': {
                    int a = op.length > 2 && op[op.length-3] == '1' ? input[i+1] : input[input[i+1]];
                    int b = op.length > 3 && op[op.length-4] == '1' ? input[i+2] : input[input[i+2]];
                    input[input[i+3]] = a * b;
                    i += 4;
                    break;
                }
                case '3':
                    input[input[i+1]] = in;
                    i += 2;
                    break;
                case '4':
                    System.out.println(input[input[i+1]]);
                    i += 2;
                    break;
                case '5': {
                    int a = op.length > 2 && op[op.length-3] == '1' ? input[i+1] : input[input[i+1]];
                    int b = op.length > 3 && op[op.length-4] == '1' ? input[i+2] : input[input[i+2]];
                    if (a != 0) i = b;
                    else i += 3;
                    break;
                }
                case '6': {
                    int a = op.length > 2 && op[op.length-3] == '1' ? input[i+1] : input[input[i+1]];
                    int b = op.length > 3 && op[op.length-4] == '1' ? input[i+2] : input[input[i+2]];
                    if (a == 0) i = b;
                    else i += 3;
                    break;
                }
                case '7': {
                    int a = op.length > 2 && op[op.length-3] == '1' ? input[i+1] : input[input[i+1]];
                    int b = op.length > 3 && op[op.length-4] == '1' ? input[i+2] : input[input[i+2]];
                    input[input[i+3]] = a < b ? 1 : 0;
                    i += 4;
                    break;
                }
                case '8': {
                    int a = op.length > 2 && op[op.length-3] == '1' ? input[i+1] : input[input[i+1]];
                    int b = op.length > 3 && op[op.length-4] == '1' ? input[i+2] : input[input[i+2]];
                    input[input[i+3]] = a == b ? 1 : 0;
                    i += 4;
                    break;
                }
            }
        }
    }
}
