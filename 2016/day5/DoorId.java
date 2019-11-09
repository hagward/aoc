import java.io.*;
import java.util.*;
import java.security.*;
import javax.xml.bind.DatatypeConverter;

public class DoorId {
    
    public void partOne() throws Exception {
        MessageDigest md = MessageDigest.getInstance("MD5");
        int found = 0;
        long i = 0;
        while (found < 8) {
            String s = "wtnhxymk" + i++;
            String hash = DatatypeConverter.printHexBinary(md.digest(s.getBytes()));
            if (hash.startsWith("00000")) {
                System.out.print(Character.toLowerCase(hash.charAt(5)));
                found++;
            }
        }
        System.out.println();
    }

    public void partTwo() throws Exception {
        MessageDigest md = MessageDigest.getInstance("MD5");
        int found = 0;
        long i = 0;
        char[] answer = new char[8];
        while (found < 8) {
            String s = "wtnhxymk" + i++;
            String hash = DatatypeConverter.printHexBinary(md.digest(s.getBytes()));
            if (hash.startsWith("00000")) {
                char pos = hash.charAt(5);
                if (pos >= '0' && pos <= '7') {
                    int poss = (int) (pos - '0');
                    if (answer[poss] == 0) {
                        answer[poss] = Character.toLowerCase(hash.charAt(6));
                        found++;
                    }
                }
            }
        }
        System.out.println(String.valueOf(answer));
    }

    public static void main(String[] args) throws Exception {
        DoorId di = new DoorId();
        di.partTwo();
    }
}