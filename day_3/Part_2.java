import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Scanner;
import java.math.BigInteger;

class Part_2{
  public static void main(String[] args){
    System.out.println("Day 3, part two!");
    File f = new File("input.txt");
    ArrayList<String> input = new ArrayList<>();



    try (Scanner sc = new Scanner(f)){
      while(sc.hasNextLine()){
        input.add(sc.nextLine());
      }
    }catch (FileNotFoundException e){
      System.out.println("Input file not found");
    }

    BigInteger total = BigInteger.ZERO;

    for (String line : input){ 
      String val = largestJoltage(line);
      total = total.add(new BigInteger(val));
    }

    System.out.println("Total: " + total.toString());

  }


  private static String largestJoltage(String line){
    ArrayList<Integer> joltages = line2JoltageList(line);

    String out = "";
    int low = 0;
    int freeDigits = joltages.size() -12;

    for (int i = 11; i >= 0; i --){
      int currentDigit = -1;
      int indexAdded = 0;
      for (int j = low; j <= low + freeDigits; j++){
        int n = joltages.get(j);
        if (n > currentDigit){
          currentDigit = n;
          indexAdded = j;
        }
      }
      freeDigits = freeDigits - (indexAdded - low);
      low = indexAdded + 1;
      out += Integer.toString(currentDigit);
    }

    return out;
  }


  private static ArrayList<Integer> line2JoltageList(String line){
    ArrayList<Integer> out = new ArrayList<>();

    for(int i = 0; i < line.length(); i++) {
      int n = line.charAt(i) - '0';
      out.add(n);
    }
    return out;
  }
}
