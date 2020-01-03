import java.io.*;
import java.util.*;

public class Day15 {
    private static final int SPACE_UNKNOWN = 0;
    private static final int SPACE_NOWALL = 1;
    private static final int SPACE_WALL = 2;
    private static final int SPACE_OXYGEN = 3;

    private static final int MOVE_WALL = 0;
    private static final int MOVE_NOWALL = 1;
    private static final int MOVE_OXYGEN = 2;

    private static final int DIR_NORTH = 1;
    private static final int DIR_SOUTH = 2;
    private static final int DIR_WEST = 3;
    private static final int DIR_EAST = 4;

    private static final int[] OPPOSITE_DIR = new int[]{-1, DIR_SOUTH, DIR_NORTH, DIR_EAST, DIR_WEST};

    public static void main(String[] args) throws IOException {
        var computer = new IntcodeComputer(Util.getCommaSeparatedInput(15));
        var space = new int[42][42];
        var dist = new int[42][42];
        var x = 21;
        var y = 21;

        reset(dist);
        space[y][x] = SPACE_NOWALL;

        // Explore the space.
        dfs(space, x, y-1, DIR_NORTH, computer);
        print(space);

        // Find the shortest path to the oxygen.
        int[] node = bfs(space, dist, x, y, SPACE_OXYGEN);
        System.out.printf("Shortest path to oxygen: %d%n", dist[node[1]][node[0]]);

        reset(dist);

        node = bfs(space, dist, node[0], node[1], -1);
        System.out.printf("Minutes until oxygen filled: %d%n", dist[node[1]][node[0]]);
    }

    private static void dfs(int[][] space, int x, int y, int direction, IntcodeComputer computer) {
        var output = (int) computer.run(new long[]{direction}, true);

        if (output == MOVE_WALL) {
            space[y][x] = SPACE_WALL;
            return;
        }

        space[y][x] = (output == MOVE_OXYGEN) ? SPACE_OXYGEN : SPACE_NOWALL;

        if (x > 0 && space[y][x-1] == SPACE_UNKNOWN) {
            dfs(space, x-1, y, DIR_WEST, computer);
        }
        if (x < space[y].length-1 && space[y][x+1] == SPACE_UNKNOWN) {
            dfs(space, x+1, y, DIR_EAST, computer);
        }
        if (y > 0 && space[y-1][x] == SPACE_UNKNOWN) {
            dfs(space, x, y-1, DIR_NORTH, computer);
        }
        if (y < space.length-1 && space[y+1][x] == SPACE_UNKNOWN) {
            dfs(space, x, y+1, DIR_SOUTH, computer);
        }

        computer.run(new long[]{OPPOSITE_DIR[direction]}, true);
    }

    private static int[] bfs(int[][] space, int[][] dist, int startX, int startY, int searchValue) {
        var q = new LinkedList<int[]>();
        q.add(new int[]{startX, startY});
        dist[startY][startX] = 0;

        while (!q.isEmpty()) {
            int[] node = q.remove();
            int x = node[0];
            int y = node[1];

            if (space[y][x] == searchValue) return new int[]{x, y};

            if (x > 0 && dist[y][x-1] == Integer.MAX_VALUE && space[y][x-1] != SPACE_WALL) {
                dist[y][x-1] = dist[y][x] + 1;
                q.add(new int[]{x-1, y});
            }
            if (x < dist[y].length-1 && dist[y][x+1] == Integer.MAX_VALUE && space[y][x+1] != SPACE_WALL) {
                dist[y][x+1] = dist[y][x] + 1;
                q.add(new int[]{x+1, y});
            }
            if (y > 0 && dist[y-1][x] == Integer.MAX_VALUE && space[y-1][x] != SPACE_WALL) {
                dist[y-1][x] = dist[y][x] + 1;
                q.add(new int[]{x, y-1});
            }
            if (y < dist.length-1 && dist[y+1][x] == Integer.MAX_VALUE && space[y+1][x] != SPACE_WALL) {
                dist[y+1][x] = dist[y][x] + 1;
                q.add(new int[]{x, y+1});
            }

            // Return the last visited node in case 'searchValue' wasn't found.
            if (q.isEmpty()) return new int[]{x, y};
        }

        // Should not happen.
        return new int[]{-1, -1};
    }

    private static void reset(int[][] dist) {
        for (var i = 0; i < dist.length; i++)
            for (var j = 0; j < dist[i].length; j++)
                dist[i][j] = Integer.MAX_VALUE;
    }

    private static void print(int[][] space) {
        for (var row : space) {
            for (var col : row) {
                if (col == SPACE_UNKNOWN) System.out.print('?');
                else if (col == SPACE_NOWALL) System.out.print(' ');
                else if (col == SPACE_WALL) System.out.print('#');
                else System.out.print('O');
            }
            System.out.println();
        }
    }
}
