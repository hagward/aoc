public class Day3 {
    public static void main(String[] args) {
        int input = 368078;

        System.out.println(distance(input));
        System.out.println(sum(input));
    }

    private static int distance(int square) {
        int direction = 0;
        int stepsToTake = 1;
        int x = 0;
        int y = 0;
        int i = 1;

        while (i < square) {
            int stepsTaken = Math.min(stepsToTake, square-i);
            i += stepsTaken;
            switch (direction) {
                case 0:
                    x += stepsTaken;
                    break;
                case 1:
                    y += stepsTaken;
                    stepsToTake += 1;
                    break;
                case 2:
                    x -= stepsTaken;
                    break;
                case 3:
                    y -= stepsTaken;
                    stepsToTake += 1;
                    break;
            }
            direction = (direction + 1) % 4;
        }

        return Math.abs(x) + Math.abs(y);
    }

    private static int sum(int square) {
        int size = 1024;
        int direction = 0;
        int stepsToTake = 1;
        int x = size/2;
        int y = size/2;
        int[][] m = new int[size][size];

        m[y][x] = 1;

        while (true) {
            for (int i = 0; i < stepsToTake; i++) {
                switch (direction) {
                    case 0: x += 1; break;
                    case 1: y -= 1; break;
                    case 2: x -= 1; break;
                    case 3: y += 1; break;
                }
                m[y][x] = m[y][x+1] + m[y-1][x+1] + m[y-1][x] + m[y-1][x-1] + m[y][x-1] + m[y+1][x-1] + m[y+1][x] + m[y+1][x+1];
                if (m[y][x] > square) {
                    return m[y][x];
                }
            }
            if (direction == 1 || direction == 3) {
                stepsToTake += 1;
            }
            direction = (direction + 1) % 4;
        }
    }
}
