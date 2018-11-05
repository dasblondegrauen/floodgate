#include <iostream>
#include "../include/floodgate.h"

using namespace std;

int main(int argc, char *argv[])
{
    if(argc < 4) {
        cout << "Usage: " << argv[0] << " <image> <host> <port> [x offset] [y offset]\n"
            << "Offsets are 0 by default" << endl;
    } else {
        int x_offset = 0, y_offset = 0;

        if(argc >= 6) {
            x_offset = stoi(argv[4]);
            y_offset = stoi(argv[5]);
        }

        cout << "Creating flood gate..." << endl;
        Floodgate fg(argv[1], argv[2], stoi(argv[3]), x_offset, y_offset);

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
