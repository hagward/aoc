import java.io.IOException;
import java.nio.file.*;

public class Util {
    public static long[] getCommaSeparatedInput(int dayNumber) throws IOException {
        String[] split = new String(Files.readAllBytes(Paths.get("inputs/day" + dayNumber + ".txt")), "UTF-8").split(",");
        long[] result = new long[split.length];
        for (int i = 0; i < split.length; i++) {
            result[i] = Long.parseLong(split[i].trim());
        }
        return result;
    }
}
