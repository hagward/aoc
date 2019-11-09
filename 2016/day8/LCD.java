import java.io.*;
import java.util.regex.*;

public class LCD {

    private static final Pattern rectPattern = Pattern.compile("^rect (\\d+)x(\\d+)$");
    private static final Pattern rowPattern = Pattern.compile("^rotate row y=(\\d+) by (\\d+)$");
    private static final Pattern columnPattern = Pattern.compile("^rotate column x=(\\d+) by (\\d+)$");

    private final int W = 50;
    private final int H = 6;

    private boolean[][] lcd = new boolean[H][W];

    public void partOne() throws IOException {
	BufferedReader in = new BufferedReader(new FileReader("input.txt"));

	String line;
	while ((line = in.readLine()) != null) {
	    Matcher matcher = rectPattern.matcher(line);
	    if (matcher.find()) {
		rect(Integer.parseInt(matcher.group(1)), Integer.parseInt(matcher.group(2)));
		continue;
	    }

	    matcher = rowPattern.matcher(line);
	    if (matcher.find()) {
		rotateRow(Integer.parseInt(matcher.group(1)), Integer.parseInt(matcher.group(2)));
		continue;
	    }

	    matcher = columnPattern.matcher(line);
	    if (matcher.find()) {
		rotateColumn(Integer.parseInt(matcher.group(1)), Integer.parseInt(matcher.group(2)));
	    }
	}

	System.out.println(numLights());
    }

    public void partTwo() {
	printLcd();
    }

    public void rect(int a, int b) {
	for (int i = 0; i < b; i++) {
	    for (int j = 0; j < a; j++) {
		lcd[i][j] = true;
	    }
	}
    }

    public void rotateRow(int a, int b) {
	boolean[] newRow = new boolean[W];
	for (int j = 0; j < W; j++) {
	    if (lcd[a][j]) {
		newRow[(j + b) % W] = true;
	    }
	}
	System.arraycopy(newRow, 0, lcd[a], 0, W);
    }

    public void rotateColumn(int a, int b) {
	boolean[] newColumn = new boolean[H];
	for (int i = 0; i < H; i++) {
	    if (lcd[i][a]) {
		newColumn[(i + b) % H] = true;
	    }
	}
	for (int i = 0; i < H; i++) {
	    lcd[i][a] = newColumn[i];
	}
    }

    public int numLights() {
	int lights = 0;
	for (int i = 0; i < H; i++) {
	    for (int j = 0; j < W; j++) {
		if (lcd[i][j]) {
		    lights++;
		}
	    }
	}
	return lights;
    }

    public void printLcd() {
	for (int i = 0; i < H; i++) {
	    for (int j = 0; j < W; j++) {
		System.out.print(lcd[i][j] ? '#' : '.');
	    }
	    System.out.println();
	}
    }

    public static void main(String[] args) throws IOException {
	LCD lcd = new LCD();
	lcd.partOne();
	lcd.partTwo();
    }
}
