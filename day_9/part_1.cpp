#include <cstdlib>
#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <stdlib.h>


using namespace std;

typedef struct {
  long long x;
  long long y;
} Coord;


// the +1 is for the specific case of this exercise
long long area(const Coord &a, const Coord &b){
  long long width = abs(a.x - b.x) + 1;
  long long height = abs(a.y - b.y) + 1;
  return width * height;
}

int main(void){
  ifstream inputFile("input.txt");
  string line;
  vector<Coord> coordinates;

  while (getline(inputFile, line)){
    auto pos = line.find(",");
    long long x = atoi(line.substr(0, pos).c_str());
    long long y = atoi(line.substr(pos+1).c_str());;
    coordinates.push_back(Coord{x, y});
  }


  long long maxArea = 0;

  for (Coord a : coordinates){
    for (Coord b : coordinates){
      long long are = area(a, b);
      if (are > maxArea){
        maxArea = are;
      }
    }
  }

  cout << "Max area: " << maxArea << endl;
}
