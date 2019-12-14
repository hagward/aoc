import java.io.IOException;

public class Day13 {
    private static final int EMPTY = 0;
    private static final int WALL = 1;
    private static final int BLOCK = 2;
    private static final int PADDLE = 3;
    private static final int BALL = 4;

    public static void main(String[] args) throws IOException {
        IntcodeComputer computer = new IntcodeComputer(Util.getCommaSeparatedInput(13));
        int[][] game = new int[100][100];
        int blocks = 0;
        while (true) {
            int x = (int) computer.run(null, true);
            int y = (int) computer.run(null, true);
            int tile = (int) computer.run(null, true);
            if (tile == -1) break;

            if (tile == BALL && game[y][x] == BLOCK) blocks--;
            else if (tile == BLOCK && game[y][x] != BLOCK) blocks++;

            game[y][x] = tile;
        }
        System.out.println(blocks);
    }
}
