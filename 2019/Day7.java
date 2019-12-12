import java.io.IOException;
import java.util.function.Consumer;

public class Day7 {
    private static long maxSignal;
    private static IntcodeComputer[] computers;

    public static void main(String[] args) throws IOException {
        long[] arr1 = new long[]{0,1,2,3,4};
        long[] arr2 = new long[]{5,6,7,8,9};
        long[] instructions = Util.getCommaSeparatedInput(7);

        computers = new IntcodeComputer[arr1.length];
        for (int i = 0; i < arr1.length; i++) {
            computers[i] = new IntcodeComputer(instructions);
        }

        maxSignal = Long.MIN_VALUE;
        permute(arr1, 0, Day7::part1);
        System.out.println(maxSignal);

        maxSignal = Long.MIN_VALUE;
        permute(arr2, 0, Day7::part2);
        System.out.println(maxSignal);
    }

    private static void part1(long[] inputs) {
        long output = 0;
        for (int i = 0; i < inputs.length; i++) {
            computers[i].reset();
            output = computers[i].run(new long[]{inputs[i], output}, true);
        }
        if (output > maxSignal) {
            maxSignal = output;
        }
    }

    private static void part2(long[] inputs) {
        long output = 0;
        for (int i = 0; i < inputs.length; i++) {
            computers[i].reset();
            output = computers[i].run(new long[]{inputs[i], output}, true);
        }
        while (output != -1) {
            for (int i = 0; i < inputs.length; i++) {
                output = computers[i].run(new long[]{output}, true);
            }
            if (output > maxSignal) {
                maxSignal = output;
            }
        }
    }

    private static void permute(long[] arr, int k, Consumer<long[]> consumer) {
        for (int i = k; i < arr.length; i++) {
            swap(arr, i, k);
            permute(arr, k+1, consumer);
            swap(arr, k, i);
        }
        if (k == arr.length -1) {
            consumer.accept(arr);
        }
    }

    private static void swap(long[] arr, int i, int j) {
        long tmp = arr[i];
        arr[i] = arr[j];
        arr[j] = tmp;
    }
}
