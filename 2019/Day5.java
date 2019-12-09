import java.io.IOException;
import java.nio.file.*;

public class Day5 {
    public static void main(String[] args) throws IOException {
        IntcodeComputer computer = new IntcodeComputer(getInput());

        System.out.println("Running with input = 1...");
        computer.run(new int[]{1});

        computer.reset();

        System.out.println("Running with input = 5...");
        computer.run(new int[]{5});
    }

    private static int[] getInput() throws IOException {
        String[] split = new String(Files.readAllBytes(Paths.get("inputs/day5.txt")), "UTF-8").split(",");
        int[] result = new int[split.length];
        for (int i = 0; i < split.length; i++) {
            result[i] = Integer.parseInt(split[i].trim());
        }
        return result;
    }
}
