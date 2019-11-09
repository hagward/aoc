import java.io.*;
import java.util.*;

public class Maze {

    private final int magicNumber;
    private int numDistinctLocations;

    public Maze(int magicNumber) {
        this.magicNumber = magicNumber;
    }

    public boolean isWall(int x, int y) {
        return (Integer.bitCount(x*x + 3*x + 2*x*y + y + y*y + magicNumber) & 1) == 1;
    }

    public void printMap(int width, int height) {
        for (int i = 0; i < height; i++) {
            for (int j = 0; j < width; j++) {
                System.out.print(isWall(j, i) ? '#' : '.');
            }
            System.out.println();
        }
    }

    public int bfs(int destX, int destY) {
        Queue<Node> queue = new LinkedList<>();
        Node[][] visited = new Node[3*destY][3*destX];

        queue.add(new Node(1, 1, 0, null));

        while (!queue.isEmpty()) {
            Node curr = queue.remove();

            if (curr.x == destX && curr.y == destY) {
                return curr.d;
            }

            visit(curr.x, curr.y-1, curr, queue, visited);
            visit(curr.x, curr.y+1, curr, queue, visited);
            visit(curr.x-1, curr.y, curr, queue, visited);
            visit(curr.x+1, curr.y, curr, queue, visited);
        }

        return -1;
    }

    public int bfs(int maxSteps) {
        numDistinctLocations = 0;

        Queue<Node> queue = new LinkedList<>();
        Node[][] visited = new Node[50][50];

        Node startNode = new Node(1, 1, 0, null);
        queue.add(startNode);
        visited[1][1] = startNode;

        while (!queue.isEmpty()) {
            Node curr = queue.remove();

            numDistinctLocations++;

            visit(maxSteps, curr.x, curr.y-1, curr, queue, visited);
            visit(maxSteps, curr.x, curr.y+1, curr, queue, visited);
            visit(maxSteps, curr.x-1, curr.y, curr, queue, visited);
            visit(maxSteps, curr.x+1, curr.y, curr, queue, visited);
        }

        return numDistinctLocations;
    }

    private void visit(int x, int y, Node curr, Queue<Node> queue, Node[][] visited) {
        if (x >= 0 && y >= 0 && visited[y][x] == null && !isWall(x, y)) {
            Node next = new Node(x, y, curr.d+1, curr);
            queue.add(next);
            visited[y][x] = next;
        }
    }

    private void visit(int maxSteps, int x, int y, Node curr, Queue<Node> queue, Node[][] visited) {
        if (curr.d < maxSteps && x >= 0 && y >= 0 && visited[y][x] == null && !isWall(x, y)) {
            Node next = new Node(x, y, curr.d+1, curr);
            queue.add(next);
            visited[y][x] = next;
        }
    }

    public static void main(String[] args) {
        Maze maze = new Maze(1352);
        // maze.printMap(10, 7);
        System.out.println(maze.bfs(50));
    }

    private static class Node {
        public final int x;
        public final int y;
        public final int d;
        public final Node p;

        public Node(int x, int y, int d, Node p) {
            this.x = x;
            this.y = y;
            this.d = d;
            this.p = p;
        }
    }
}