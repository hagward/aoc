import java.io.*;
import java.nio.file.*;

public class Day8 {
    public static void main(String[] args) throws IOException {
        var input = getInput();
        var width = 25;
        var height = 6;
        part1(input, width, height);
        part2(input, width, height);
    }

    static void part1(String input, int width, int height) {
        var minZeros = Long.MAX_VALUE;
        var minZerosLayer = "";

        // Find layer with fewest 0 digits.
        for (var i = 0; i < input.length();) {
            var layer = input.substring(i, i + width * height);
            var zeros = layer.chars().filter(c -> c == (int)'0').count();
            if (zeros < minZeros) {
                minZeros = zeros;
                minZerosLayer = layer;
            }
            i += width * height;
        }

        var ones = minZerosLayer.chars().filter(c -> c == (int)'1').count();
        var twos = minZerosLayer.chars().filter(c -> c == (int)'2').count();

        System.out.println(ones * twos);
    }

    static void part2(String input, int width, int height) {
        var layerSize = width * height;
        var numLayers = input.length() / layerSize;
        var layers = new char[numLayers][layerSize];

        // Read image data, layer by layer.
        for (var i = 0; i < numLayers; i++) {
            layers[i] = input.substring(i * layerSize, (i + 1) * layerSize).toCharArray();
        }

        // Merge layers to get the resulting image.
        var result = new char[layerSize];
        loop: for (var i = 0; i < layerSize; i++) {
            for (var j = 0; j < numLayers; j++) {
                if (layers[j][i] != '2') {
                    result[i] = layers[j][i];
                    continue loop;
                }
            }
        }

        // Print the image.
        for (var i = 0; i < height; i++) {
            for (var j = 0; j < width; j++) {
                var c = result[i * width + j];
                System.out.print(c == '1' ? '*' : ' ');
            }
            System.out.println();
        }
    }

    static String getInput() throws IOException {
        return new String(Files.readAllBytes(Paths.get("inputs/day8.txt")), "UTF-8").trim();
    }
}
