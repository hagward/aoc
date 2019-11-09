import java.io.*;
import java.util.*;

public class TaxiCab {

    private static final String INPUT = "L1, L3, L5, L3, R1, L4, L5, R1, R3, L5, R1, L3, L2, L3, R2, R2, L3, L3, R1, L2, R1, L3, L2, R4, R2, L5, R4, L5, R4, L2, R3, L2, R4, R1, L5, L4, R1, L2, R3, R1, R2, L4, R1, L2, R3, L2, L3, R5, L192, R4, L5, R4, L1, R4, L4, R2, L5, R45, L2, L5, R4, R5, L3, R5, R77, R2, R5, L5, R1, R4, L4, L4, R2, L4, L1, R191, R1, L1, L2, L2, L4, L3, R1, L3, R1, R5, R3, L1, L4, L2, L3, L1, L1, R5, L4, R1, L3, R1, L2, R1, R4, R5, L4, L2, R4, R5, L1, L2, R3, L4, R2, R2, R3, L2, L3, L5, R3, R1, L4, L3, R4, R2, R2, R2, R1, L4, R4, R1, R2, R1, L2, L2, R4, L1, L2, R3, L3, L5, L4, R4, L3, L1, L5, L3, L5, R5, L5, L4, L2, R1, L2, L4, L2, L4, L1, R4, R4, R5, R1, L4, R2, L4, L2, L4, R2, L4, L1, L2, R1, R4, R3, R2, R2, R5, L1, L2";

    public static void main(String[] args) {
        String[] commands;
        if (args.length > 1) {
            commands = args[1].split(", ");
        } else {
            commands = INPUT.split(", ");
        }

        if (args[0].equals("p1")) {
            part1(commands);
        } else if (args[0].equals("p2")) {
            part2(commands);
        }
    }

    private static void part1(String[] commands) {
        int dir = 0;
        int x = 0;
        int y = 0;

        for (String command : commands) {
            if (command.charAt(0) == 'L') {
                dir = Math.floorMod(dir - 1, 4);
            } else {
                dir = Math.floorMod(dir + 1, 4);
            }

            int steps = Integer.valueOf(command.substring(1));

            switch (dir) {
                case 0: y -= steps; break;
                case 1: x += steps; break;
                case 2: y += steps; break;
                case 3: x -= steps; break;
            }
        }

        int distance = Math.abs(x) + Math.abs(y);

        System.out.println("x: " + x);
        System.out.println("y: " + y);
        System.out.println("d: " + distance);
    }

    private static void part2(String[] commands) {
        boolean[][] visited = new boolean[500][500];
        visited[250][250] = true;

        int dir = 0;
        int x = 0;
        int y = 0;

        commands: for (String command : commands) {
            if (command.charAt(0) == 'L') {
                dir = Math.floorMod(dir - 1, 4);
            } else {
                dir = Math.floorMod(dir + 1, 4);
            }

            int steps = Integer.valueOf(command.substring(1));

            for (int i = 0; i < steps; i++) {
                switch (dir) {
                    case 0: y--; break;
                    case 1: x++; break;
                    case 2: y++; break;
                    case 3: x--; break;
                }

                if (visited[y + 250][x + 250]) {
                    break commands;
                } else {
                    visited[y + 250][x + 250] = true;
                }
            }
        }

        int distance = Math.abs(x) + Math.abs(y);

        System.out.println("x: " + x);
        System.out.println("y: " + y);
        System.out.println("d: " + distance);
    }

}