import java.io.IOException;

public class Day5 {
    public static void main(String[] args) throws IOException {
        IntcodeComputer computer = new IntcodeComputer(Util.getCommaSeparatedInput(5));

        System.out.println("Running with input = 1...");
        computer.run(new long[]{1});

        computer.reset();

        System.out.println("Running with input = 5...");
        computer.run(new long[]{5});
    }
}
