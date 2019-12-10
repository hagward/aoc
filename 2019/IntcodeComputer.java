import java.util.Arrays;

public class IntcodeComputer {
    private static final int MEM_SIZE = 1024;

    private static final int MODE_POSITION = 0;
    private static final int MODE_IMMEDIATE = 1;
    private static final int MODE_RELATIVE = 2;

    private final long[] instructions;
    private int ip;
    private int rp;

    public long[] mem;

    public IntcodeComputer(long[] instructions) {
        this.instructions = instructions;
        reset();
    }

    public void reset() {
        mem = Arrays.copyOf(instructions, MEM_SIZE);
        ip = 0;
        rp = 0;
    }

    public long run() {
        return run(null, false);
    }

    public long run(long[] input) {
        return run(input, false);
    }

    public long run(long[] input, boolean returnOnOutput) {
        int curInput = 0;

        while (ip < mem.length) {
            int opcode = (int) mem[ip] % 100;

            if (opcode == 99) return -1;

            int modeX = (int) (mem[ip] / 100) % 10;
            int modeY = (int) (mem[ip] / 1000) % 10;
            long x = getParam(0, modeX);
            long y = getParam(1, modeY);

            switch (opcode) {
                case 1: {
                    mem[(int) mem[ip+3]] = x + y;
                    ip += 4;
                    break;
                }
                case 2: {
                    mem[(int) mem[ip+3]] = x * y;
                    ip += 4;
                    break;
                }
                case 3:
                    mem[(int) x] = input[curInput];
                    if (curInput < input.length - 1) curInput++;
                    ip += 2;
                    break;
                case 4:
                    if (returnOnOutput) return x;
                    System.out.println(x);
                    ip += 2;
                    break;
                case 5: {
                    if (x != 0) ip = (int) y;
                    else ip += 3;
                    break;
                }
                case 6: {
                    if (x == 0) ip = (int) y;
                    else ip += 3;
                    break;
                }
                case 7: {
                    mem[(int) mem[ip+3]] = x < y ? 1 : 0;
                    ip += 4;
                    break;
                }
                case 8: {
                    mem[(int) mem[ip+3]] = x == y ? 1 : 0;
                    ip += 4;
                    break;
                }
                case 9: {
                    rp += x;
                    ip += 2;
                    break;
                }
                default: {
                    throw new RuntimeException("Unknown opcode: " + opcode);
                }
            }
        }
        return -1;
    }

    private long getParam(int i, int mode) {
        int address = ip + i + 1;
        if (address >= mem.length) return -1;
        long value = mem[address];

        switch (mode) {
            case MODE_POSITION: {
                if (value >= 0 && value < mem.length) {
                    return mem[(int) value];
                }
                break;
            }
            case MODE_IMMEDIATE: {
                return value;
            }
            case MODE_RELATIVE: {
                int addr = (int) (rp + value);
                if (addr >= 0 && addr < mem.length) {
                    return mem[addr];
                }
                break;
            }
            default: {
                throw new RuntimeException("Unknown mode: " + mode);
            }
        }

        return -1;
    }
}
