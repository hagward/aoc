import java.io.*;
import java.nio.file.*;
import java.util.*;
import java.util.stream.*;

public class Day6 {
    public static void main(String[] args) throws IOException {
        List<String> lines = Files
            .lines(Paths.get("inputs/day6.txt"))
            .collect(Collectors.toList());

        Map<String, Node> nodes = new HashMap<>();
        Node parentNode = null;

        for (String line : lines) {
            String[] split = line.split("\\)");
            String parent = split[0];
            String child = split[1];

            parentNode = nodes.get(parent);
            if (parentNode == null) {
                parentNode = new Node(parent);
                nodes.put(parent, parentNode);
            }

            Node childNode = nodes.get(child);
            if (childNode == null) {
                childNode = new Node(child);
                nodes.put(child, childNode);
            }

            parentNode.children.add(childNode);
            childNode.parent = parentNode;
        }

        Node rootNode = findRootNode(parentNode);

        int answer = 0;
        Queue<Node> queue = new LinkedList<>();
        queue.add(rootNode);
        while (!queue.isEmpty()) {
            Node currNode = queue.remove();

            answer += currNode.depth;

            for (Node childNode : currNode.children) {
                childNode.depth = currNode.depth + 1;
                queue.add(childNode);
            }
        }

        System.out.println(answer);

        Node youNode = nodes.get("YOU");
        youNode.visited = true;
        queue = new LinkedList<>();
        queue.add(youNode);
        while (!queue.isEmpty()) {
            Node currNode = queue.remove();

            if (currNode.label.equals("SAN")) {
                System.out.println(currNode.transfers - 2);
                break;
            }

            for (Node childNode : currNode.children) {
                if (!childNode.visited) {
                    childNode.transfers = currNode.transfers + 1;
                    childNode.visited = true;
                    queue.add(childNode);
                }
            }
            if (currNode.parent != null && !currNode.parent.visited) {
                currNode.parent.transfers = currNode.transfers + 1;
                currNode.parent.visited = true;
                queue.add(currNode.parent);
            }
        }
    }

    private static Node findRootNode(Node startNode) {
        Node currNode = startNode;
        while (currNode.parent != null) {
            currNode = currNode.parent;
        }
        return currNode;
    }

    static class Node {
        String label;
        Node parent;
        int depth = 0;
        int transfers = 0;
        boolean visited = false;
        List<Node> children = new ArrayList<>();

        Node(String label) {
            this.label = label;
        }
    }
}
