import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Scanner;

class Part_1{
  public static void main(String[] args){
    System.out.println("Day 3, part one!");
    File f = new File("input.txt");
    ArrayList<String> input = new ArrayList<>();



    try (Scanner sc = new Scanner(f)){
      while(sc.hasNextLine()){
        input.add(sc.nextLine());
      }
    }catch (FileNotFoundException e){
      System.out.println("Input file not found");
    }

    int total = 0;

    for (String line : input){ 
      total += largestJoltage(line);

    }

    System.out.println("Total: " + Integer.toString(total));

  }


  private static int largestJoltage(String line){
    ArrayList<Integer> joltages = line2JoltageList(line);
    int firstDigit = 0;
    int firstDigitIndex = 0;

    for (int i = 0; i < joltages.size() - 1; i ++){
      int n = joltages.get(i);
      if (n > firstDigit){
        firstDigit = n;
        firstDigitIndex = i;
      }
    }


    int secondDigit = 0;
    for (int i = firstDigitIndex + 1; i < joltages.size(); i++){
      int n = joltages.get(i);
      if (n > secondDigit){
        secondDigit = n;
      }
    }


    return firstDigit * 10 + secondDigit;

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
