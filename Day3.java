public class Day3 {
    public static void main(String[] args) {
        int input = 368078;

        int direction = 0;
        int stepsToTake = 1;
        int x = 0;
        int y = 0;
        int i = 1;

        while (i < input) {
            int stepsTaken = Math.min(stepsToTake, input-i);
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

        System.out.println(Math.abs(x) + Math.abs(y));
    }
}
