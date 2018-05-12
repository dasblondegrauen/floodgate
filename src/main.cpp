#include <iostream>
#include "../include/floodgate.h"

using namespace std;

int main(int argc, char *argv[])
{
  if(argc < 4) {
    cout << "Usage: " << argv[0] << "<image> <host> <port>" << endl;
  } else {
    int xOffset = 0, yOffset = 0;

    if(argc >= 6) {
      xOffset = stoi(argv[4]);
      yOffset = stoi(argv[5]);
    }

    cout << "Creating flood gate..." << endl;
    Floodgate fg(argv[1], argv[2], stoi(argv[3]), xOffset, yOffset);

    cout << "Opening flood gate..." << endl;
    if(!fg.open()) {
      cout << "Failed to open flood gate." << endl;
      return 1;
    }

    cout << "FLOODING!" << endl;
    fg.flood();
  }
  return 0;
}
