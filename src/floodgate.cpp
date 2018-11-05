#include <sstream>
#include <iomanip>
#include <unistd.h>
#include <errno.h>
#include "../include/floodgate.h"

Floodgate::Floodgate(string image, string host, uint16_t port, int x_offset, int y_offset) :
    host(host),
    port(port) {

    cv::Mat image_mat = cv::imread(image);
    stringstream str;
    for(int y = 0; y < image_mat.size().height; y++) {
        for(int x = 0; x < image_mat.size().width; x++) {
            str << "PX " << dec << x+x_offset << " " << y+y_offset << " "
                << b2h(image_mat.at<cv::Vec3b>(y, x)[0])
                    << b2h(image_mat.at<cv::Vec3b>(y, x)[1])
                    << b2h(image_mat.at<cv::Vec3b>(y, x)[2])
                    << "\n";
        }
    }

    command = str.str();
}

Floodgate::~Floodgate() {
    shut();
}

bool Floodgate::open() {
    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if(sockfd == -1) {
        cerr << "Could not create socket: " << errno << endl;
        return false;
    }

    server.sin_addr.s_addr = inet_addr(host.c_str());
    server.sin_family = AF_INET;
    server.sin_port = htons(port);

    if(connect(sockfd, (struct sockaddr*) &server, sizeof(struct sockaddr_in))) {
        cerr << "Could not connect: " << errno << endl;
        return false;
    }

    return true;
}

void Floodgate::flood() {
    const char* data = command.data();
    const size_t len = command.length();
    size_t offset = 0;

    while(true) {
        size_t written = write(sockfd, data+offset, len-offset);
        offset += written;

        if(offset >= len) {
            offset = 0;
        }
    }

    cerr << "Flood loop interrupted: " << errno << endl;
}


void Floodgate::shut() {
    if(close(sockfd) == -1) {
        cerr << "Could not close socket: " << errno << endl;
    }
}

string Floodgate::b2h(uint8_t value) {
    char buf[3];
    sprintf(buf, "%02X", value);
    return string(buf);
}
