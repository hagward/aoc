import java.io.*;
import java.nio.file.*;
import java.util.stream.*;

public class Day1 {
    public static void main(String[] args) throws IOException {
        System.out.println(getMasses().map(Day1::calculateFuel).sum());
        System.out.println(getMasses().map(Day1::calculateFuelRec).sum());
    }

    private static LongStream getMasses() throws IOException {
        return Files
            .lines(Paths.get("inputs/day1.txt"))
            .mapToLong(Long::parseLong);
    }

    private static long calculateFuel(long mass) {
        return mass / 3 - 2;
    }

    private static long calculateFuelRec(long mass) {
        long fuel = calculateFuel(mass);
        if (fuel <= 0) return 0;
        return fuel + calculateFuelRec(fuel);
    }
}
