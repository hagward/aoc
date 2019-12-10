import java.io.IOException;
import java.nio.file.*;

public class Day9 {
    public static void main(String[] args) throws IOException {
        IntcodeComputer computer = new IntcodeComputer(getInput());
        computer.run(new long[]{1});
        computer.reset();
        computer.run(new long[]{2});
    }

    private static long[] getInput() throws IOException {
        String[] split = new String(Files.readAllBytes(Paths.get("inputs/day9.txt")), "UTF-8").split(",");
        long[] result = new long[split.length];
        for (int i = 0; i < split.length; i++) {
            result[i] = Long.parseLong(split[i].trim());
        }
        return result;
    }
}
