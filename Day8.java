import java.io.*;
import java.nio.file.*;
import java.util.*;

public class Day8 {
    private static class Cpu {
        private final Map<String, Integer> registers = new HashMap<>();
        private int highestValue = Integer.MIN_VALUE;

        private void execute(String instruction) {
            String[] parts = instruction.split(" ");
            if (evaluateExpression(parts[4], parts[5], Integer.parseInt(parts[6]))) {
                evaluateAssignment(parts[0], parts[1], Integer.parseInt(parts[2]));
            }
        }

        private boolean evaluateExpression(String register, String operator, int rhs) {
            int lhs = registers.getOrDefault(register, 0);
            switch (operator) {
                case ">": return lhs > rhs;
                case "<": return lhs < rhs;
                case ">=": return lhs >= rhs;
                case "<=": return lhs <= rhs;
                case "==": return lhs == rhs;
                case "!=": return lhs != rhs;
                default: throw new RuntimeException("Unknown operator " + operator);
            }
        }

        private void evaluateAssignment(String register, String operator, int rhs) {
            int lhs = registers.getOrDefault(register, 0);
            switch (operator) {
                case "inc": registers.put(register, lhs + rhs); break;
                case "dec": registers.put(register, lhs - rhs); break;
                default: throw new RuntimeException("Unknown operator " + operator);
            }

            int newValue = registers.get(register);
            if (newValue > highestValue) {
                highestValue = newValue;
            }
        }
    }

    public static void main(String[] args) throws IOException {
        Cpu cpu = new Cpu();
        Files.lines(Paths.get("Day8Input.txt")).forEach(cpu::execute);
        cpu.registers.values().stream().mapToInt(Integer::intValue).max().ifPresent(System.out::println);
        System.out.println(cpu.highestValue);
    }
}
