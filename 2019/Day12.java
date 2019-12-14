import java.util.Arrays;

public class Day12 {
    public static void main(String[] args) {
        part1();
        part2();
    }

    private static void part1() {
        Moon[] m = getInput();

        for (int i = 0; i < 1000; i++) step(m);

        int totalEnergy = Arrays.stream(m).mapToInt(Moon::getEnergy).sum();
        System.out.println("Total energy: " + totalEnergy);
    }

    private static void part2() {
        Moon[] m1 = getInput();
        Moon[] m2 = getInput();
        int[] cycles = new int[3];
        int steps = 1;

        while (cycles[0] == 0 || cycles[1] == 0 || cycles[2] == 0) {
            step(m2);
            steps++;
            for (int i = 0; i < 3; i++) {
                if (cycles[i] == 0 && m1[0].dim[i] == m2[0].dim[i] && m1[1].dim[i] == m2[1].dim[i] && m1[2].dim[i] == m2[2].dim[i] && m1[3].dim[i] == m2[3].dim[i]) {
                    cycles[i] = steps;
                }
            }
        }

        System.out.printf("Use WolframAlpha: lcm(%d,%d,%d)", cycles[0], cycles[1], cycles[2]);
    }

    private static void step(Moon[] moons) {
        for (Moon m1 : moons) {
            for (Moon m2 : moons) {
                for (int i = 0; i < 3; i++) {
                    if (m1.dim[i] > m2.dim[i]) m1.vel[i]--;
                    else if (m1.dim[i] < m2.dim[i]) m1.vel[i]++;
                }
            }
        }

        for (Moon m : moons) {
            for (int i = 0; i < 3; i++) {
                m.dim[i] += m.vel[i];
            }
        }
    }

    private static Moon[] getInput() {
        return new Moon[] {
            new Moon(-10, -10, -13),
            new Moon(5, 5, -9),
            new Moon(3, 8, -16),
            new Moon(1, 3, -3)
        };
    }

    private static class Moon {
        int[] dim;
        int[] vel;

        public Moon(int x, int y, int z) {
            dim = new int[]{x, y, z};
            vel = new int[]{0, 0, 0};
        }

        public int getEnergy() {
            int potential = Arrays.stream(dim).map(Math::abs).sum();
            int kinetic = Arrays.stream(vel).map(Math::abs).sum();
            return potential * kinetic;
        }

        @Override
        public String toString() {
            return "pos=" + Arrays.toString(dim) + ", vel=" + Arrays.toString(vel) + ", energy=" + getEnergy();
        }
    }
}
