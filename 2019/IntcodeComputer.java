import java.util.Arrays;

public class IntcodeComputer {
    private static final int MODE_POSITION = 0;
    private static final int MODE_IMMEDIATE = 1;

    private final int[] instructions;
    private int ip;

    public int[] mem;

    public IntcodeComputer(int[] instructions) {
        this.instructions = instructions;
        reset();
    }

    public void reset() {
        this.mem = Arrays.copyOf(instructions, instructions.length);
    }

    public int run() {
        return run(null, false);
    }

    public int run(int[] input) {
        return run(input, false);
    }

    public int run(int[] input, boolean returnOnOutput) {
        int curInput = 0;

        while (ip < mem.length) {
            int opCode = mem[ip] % 100;

            if (opCode == 99) return -1;

            int modeX = (mem[ip] / 100) % 10;
            int modeY = (mem[ip] / 1000) % 10;
            int x = getParam(0, modeX);
            int y = getParam(1, modeY);

            switch (opCode) {
                case 1: {
                    mem[mem[ip+3]] = x + y;
                    ip += 4;
                    break;
                }
                case 2: {
                    mem[mem[ip+3]] = x * y;
                    ip += 4;
                    break;
                }
                case 3:
                    mem[mem[ip+1]] = input[curInput];
                    if (curInput < input.length - 1) curInput++;
                    ip += 2;
                    break;
                case 4:
                    if (returnOnOutput) return mem[mem[ip+1]];
                    System.out.println(mem[mem[ip+1]]);
                    ip += 2;
                    break;
                case 5: {
                    if (x != 0) ip = y;
                    else ip += 3;
                    break;
                }
                case 6: {
                    if (x == 0) ip = y;
                    else ip += 3;
                    break;
                }
                case 7: {
                    mem[mem[ip+3]] = x < y ? 1 : 0;
                    ip += 4;
                    break;
                }
                case 8: {
                    mem[mem[ip+3]] = x == y ? 1 : 0;
                    ip += 4;
                    break;
                }
            }
        }
        return -1;
    }

    private int getParam(int i, int mode) {
        int address = ip + i + 1;
        if (address >= mem.length) return -1;
        int value = mem[address];
        if (mode == MODE_POSITION && value >= mem.length) return -1;
        return mode == MODE_IMMEDIATE ? value : mem[value];
    }
}
