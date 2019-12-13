import java.util.Arrays;

public class Day12 {
    public static void main(String[] args) {
        Moon[] moons = new Moon[] {
            new Moon(-10, -10, -13),
            new Moon(5, 5, -9),
            new Moon(3, 8, -16),
            new Moon(1, 3, -3)
        };

        for (int i = 0; i < 1000; i++) step(moons);

        int totalEnergy = Arrays.stream(moons).mapToInt(Moon::getEnergy).sum();
        for (Moon m : moons) System.out.println(m);
        System.out.println("Total energy: " + totalEnergy);
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
