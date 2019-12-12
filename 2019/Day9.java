import java.io.IOException;

public class Day9 {
    public static void main(String[] args) throws IOException {
        IntcodeComputer computer = new IntcodeComputer(Util.getCommaSeparatedInput(9));
        computer.run(new long[]{1});
        computer.reset();
        computer.run(new long[]{2});
    }
}
