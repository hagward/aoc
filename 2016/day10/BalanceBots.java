import java.io.*;
import java.util.*;
import java.util.stream.*;

public class BalanceBots {

    private int[][] bots = new int[210][2];
    private int[] outputs = new int[21];

    public void solve() throws IOException {
	List<String> lines = new BufferedReader(new FileReader("input.txt"))
		.lines()
		.collect(Collectors.toList());

	int executed = 0;
	while (true) {
	    for (int i = 0; i < lines.size(); i++) {
		String line = lines.get(i);
		if (line == null) continue;
		if (parseLine(line)) {
		    executed++;
		    lines.set(i, null);
		}
		if (executed == lines.size()) {
		    System.out.println(outputs[0] * outputs[1] * outputs[2]);
		    return;
		}
	    }
	}
    }

    private boolean parseLine(String line) {
	String[] parts = line.split(" ");
	if (parts[0].equals("bot")) {
	    int giver = Integer.parseInt(parts[1]);
	    if (canBotProceed(giver)) {
		int receiverLow = Integer.parseInt(parts[6]);
		int receiverHigh = Integer.parseInt(parts[11]);

		if (parts[5].equals("bot")) {
		    giveValueBotToBot(giver, receiverLow, false);
		} else {
		    giveValueBotToOutput(giver, receiverLow, false);
		}

		if (parts[10].equals("bot")) {
		    giveValueBotToBot(giver, receiverHigh, true);
		} else {
		    giveValueBotToOutput(giver, receiverHigh, true);
		}

		return true;
	    } else {
		return false;
	    }
	} else {
	    giveValueToBot(Integer.parseInt(parts[1]), Integer.parseInt(parts[5]));
	    return true;
	}
    }

    private boolean canBotProceed(int bot) {
	return bots[bot][0] != 0 && bots[bot][1] != 0;
    }

    private void giveValueBotToBot(int giver, int receiver, boolean high) {
	int valueIndex = (high) ? getHighIndex(giver) : getLowIndex(giver);
	giveValueToBot(bots[giver][valueIndex], receiver);
	bots[giver][valueIndex] = 0;
    }

    private int getHighIndex(int bot) {
	if ((bots[bot][0] == 61 && bots[bot][1] == 17) || (bots[bot][1] == 61 && bots[bot][0] == 17)) {
	    System.out.printf("bot %d just compared 61 to 17!%n", bot);
	}
	return (bots[bot][0] > bots[bot][1] && bots[bot][0] > 0) ? 0 : 1;
    }

    private int getLowIndex(int bot) {
	if ((bots[bot][0] == 61 && bots[bot][1] == 17) || (bots[bot][1] == 61 && bots[bot][0] == 17)) {
	    System.out.printf("bot %d just compared 61 to 17!%n", bot);
	}
	return (bots[bot][0] < bots[bot][1] && bots[bot][0] > 0) ? 0 : 1;
    }

    private void giveValueToBot(int value, int bot) {
	int valueIndex = (bots[bot][0] == 0) ? 0 : 1;
	bots[bot][valueIndex] = value;
    }

    private void giveValueBotToOutput(int bot, int output, boolean high) {
	int valueIndex = (high) ? getHighIndex(bot) : getLowIndex(bot);
	outputs[output] = bots[bot][valueIndex];
	bots[bot][valueIndex] = 0;
    }

    public void printDebug() {
	for (int i = 0; i < bots.length; i++) {
	    System.out.printf("bot %d: %d %d%n", i, bots[i][0], bots[i][1]);
	}
	for (int i = 0; i < outputs.length; i++) {
	    System.out.printf("output %d: %d%n", i, outputs[i]);
	}
    }

    public static void main(String[] args) throws IOException {
	BalanceBots bb = new BalanceBots();
	bb.solve();
    }
}
