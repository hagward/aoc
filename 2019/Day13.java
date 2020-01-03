import java.io.IOException;

public class Day13 {
    private static final int TILE_BLOCK = 2;
    private static final int TILE_PADDLE = 3;
    private static final int TILE_BALL = 4;

    private static final long JOYSTICK_NEUTRAL = 0;
    private static final long JOYSTICK_LEFT = -1;
    private static final long JOYSTICK_RIGHT = 1;

    public static void main(String[] args) throws IOException {
        var computer = new IntcodeComputer(Util.getCommaSeparatedInput(13));
        part1(computer);
        computer.reset();
        part2(computer);
    }

    private static void part1(IntcodeComputer computer) {
        var game = new int[100][100];
        var blocks = 0;

        while (true) {
            var x = (int) computer.run(null, true);
            var y = (int) computer.run(null, true);
            var tile = (int) computer.run(null, true);

            if (tile == -1) break;

            if (tile == TILE_BALL && game[y][x] == TILE_BLOCK) blocks--;
            else if (tile == TILE_BLOCK && game[y][x] != TILE_BLOCK) blocks++;

            game[y][x] = tile;
        }

        System.out.println(blocks);
    }

    private static void part2(IntcodeComputer computer) {
        computer.mem[0] = 2;

        var game = new int[100][100];
        var input = new long[]{JOYSTICK_NEUTRAL};
        var score = 0;
        var paddleX = 0;

        while (true) {
            var x = (int) computer.run(input, true);
            var y = (int) computer.run(input, true);
            var z = (int) computer.run(input, true);

            if (z == -1) break;

            if (x == -1 && y == 0) {
                score = z;
            } else {
                var tile = z;

                if (tile == TILE_PADDLE) {
                    paddleX = x;
                } else if (tile == TILE_BALL) {
                    if (x < paddleX) input[0] = JOYSTICK_LEFT;
                    else if (x > paddleX) input[0] = JOYSTICK_RIGHT;
                    else input[0] = JOYSTICK_NEUTRAL;
                }

                game[y][x] = tile;
            }
        }

        System.out.println(score);
    }
}
