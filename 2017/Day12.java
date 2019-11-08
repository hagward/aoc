import java.io.*;
import java.nio.file.*;
import java.util.*;

public class Day12 {
    public static void main(String[] args) throws IOException {
        List<List<Integer>> graph = new ArrayList<>();
        Files.lines(Paths.get("Day12Input.txt"))
                .forEach(line -> {
                    String[] parts = line.split(" <-> ");
                    String[] pipes = parts[1].split(", ");
                    List<Integer> newList = new ArrayList<>();
                    graph.add(newList);
                    for (String pipe : pipes) {
                        newList.add(Integer.parseInt(pipe));
                    }
                });

        System.out.println(groupSize(graph));
        System.out.println(groupCount(graph));
    }

    private static int groupSize(List<List<Integer>> graph) {
        int size = 0;

        Queue<Integer> queue = new LinkedList<>();
        Set<Integer> visited = new HashSet<>();

        queue.add(0);

        while (!queue.isEmpty()) {
            Integer k = queue.remove();

            if (visited.add(k)) {
                size++;
            }

            for (Integer i : graph.get(k)) {
                if (!visited.contains(i)) {
                    queue.add(i);
                }
            }
        }

        return size;
    }

    private static int groupCount(List<List<Integer>> graph) {
        int count = 0;

        Queue<Integer> queue = new LinkedList<>();
        Set<Integer> visited = new HashSet<>();

        for (int i = 0; i < graph.size(); i++) {
            queue.add(i);

            if (visited.contains(i)) {
                continue;
            }

            count++;

            while (!queue.isEmpty()) {
                Integer k = queue.remove();

                visited.add(k);

                for (Integer m : graph.get(k)) {
                    if (!visited.contains(m)) {
                        queue.add(m);
                    }
                }
            }
        }

        return count;
    }
}
