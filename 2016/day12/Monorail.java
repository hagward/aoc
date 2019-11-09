import java.io.*;
import java.util.*;
import java.util.stream.*;

public class Monorail {

    private int[] reg = new int[4];
    public List<String> code;

    public Monorail() throws IOException {
	reg[2] = 1;
	code = new BufferedReader(new FileReader("input.txt"))
		.lines()
		.collect(Collectors.toList());
    }

    public void execute() {
	int pc = 0;
	while (pc < code.size()) {
	    String[] parts = code.get(pc).split(" ");

	    switch (parts[0]) {
		case "cpy": {
		    int x = (isRegister(parts[1].charAt(0)))
			    ? reg[parts[1].charAt(0)-'a']
			    : Integer.parseInt(parts[1]);
		    reg[parts[2].charAt(0)-'a'] = x;
		}
		break;

		case "dec":
		    reg[parts[1].charAt(0)-'a']--;
		    break;

		case "inc":
		    reg[parts[1].charAt(0)-'a']++;
		    break;

		case "jnz": {
		    int x = (isRegister(parts[1].charAt(0)))
			    ? reg[parts[1].charAt(0)-'a']
			    : Integer.parseInt(parts[1]);
		    if (x != 0) {
			pc += Integer.parseInt(parts[2]) - 1;
		    }
		}
	    }

	    pc++;
	}
    }

    private boolean isRegister(char c) {
	return c >= 'a' && c <= 'd';
    }

    public void printRegisters() {
	System.out.println("Registers: " + Arrays.toString(reg));
    }

    public static void main(String[] args) throws IOException {
	Monorail m = new Monorail();
	m.execute();
	m.printRegisters();
    }

}
