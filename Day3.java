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
        int direction = 0;
        int stepsToTake = 1;
        int x = 0;
        int y = 0;
        int[][] m = new int[1024][1024];
        int p = 512;

        m[p][p] = 1;

        while (true) {
            for (int i = 0; i < stepsToTake; i++) {
                switch (direction) {
                    case 0: x += 1; break;
                    case 1: y -= 1; break;
                    case 2: x -= 1; break;
                    case 3: y += 1; break;
                }
                m[y+p][x+p] = m[y+p][x+p+1] + m[y+p-1][x+p+1] + m[y+p-1][x+p] + m[y+p-1][x+p-1] + m[y+p][x+p-1] + m[y+p+1][x+p-1] + m[y+p+1][x+p] + m[y+p+1][x+p+1];
                if (m[y+p][x+p] > square) {
                    return m[y+p][x+p];
                }
            }
            if (direction == 1 || direction == 3) {
                stepsToTake += 1;
            }
            direction = (direction + 1) % 4;
        }
    }
}
