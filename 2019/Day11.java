import java.io.IOException;

public class Day11 {
    private static final int hullSize = 1000;
    public static void main(String[] args) throws IOException {
        IntcodeComputer computer = new IntcodeComputer(Util.getCommaSeparatedInput(11));

        int[][] hull = new int[hullSize][hullSize];
        boolean[][] painted = new boolean[hullSize][hullSize];
        runRobot(computer, hull, painted);
        System.out.println(getNumPaintedAtLeastOnce(painted));

        hull = new int[hullSize][hullSize];
        hull[hullSize/2][hullSize/2] = 1;
        computer.reset();
        int[] dim = runRobot(computer, hull, painted);
        for (int i = dim[2]; i <= dim[3]; i++) {
            for (int j = dim[0]; j <= dim[1]; j++) {
                System.out.print(hull[i][j] == 1 ? '#' : ' ');
            }
            System.out.println();
        }
    }

    private static int[] runRobot(IntcodeComputer computer, int[][] hull, boolean[][] painted) {
        Robot robot = new Robot();
        int minX = robot.x;
        int maxX = robot.x;
        int minY = robot.y;
        int maxY = robot.y;

        while (true) {
            long color = computer.run(new long[]{hull[robot.y][robot.x]}, true);
            if (color == -1) break;
            long turnDir = computer.run(new long[]{hull[robot.y][robot.x]}, true);

            if (robot.x < minX) minX = robot.x;
            if (robot.x > maxX) maxX = robot.x;
            if (robot.y < minY) minY = robot.y;
            if (robot.y > maxY) maxY = robot.y;

            hull[robot.y][robot.x] = (int) color;
            painted[robot.y][robot.x] = true;

            if (turnDir == 0) robot.turnLeft();
            else robot.turnRight();
            robot.moveForward();
        }

        return new int[]{minX, maxX, minY, maxY};
    }

    private static int getNumPaintedAtLeastOnce(boolean[][] painted) {
        int numPainted = 0;
        for (boolean[] row : painted)
            for (boolean col : row)
                if (col) numPainted++;
        return numPainted;
    }

    private static class Robot {
        int x = hullSize / 2;
        int y = hullSize / 2;
        int dir = 90;

        public void turnLeft() {
            dir = (dir + 90) % 360;
        }

        public void turnRight() {
            dir = (dir + 270) % 360;
        }

        public void moveForward() {
            switch (dir) {
                case 0:   x += 1; break;
                case 90:  y -= 1; break;
                case 180: x -= 1; break;
                case 270: y += 1; break;
            }
        }
    }
}
