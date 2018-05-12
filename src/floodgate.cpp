#include <sstream>
#include <iomanip>
#include <unistd.h>
#include "../include/floodgate.h"

Floodgate::Floodgate(string image, string host, uint16_t port, int xOffset, int yOffset) :
  host(host),
  port(port) {

  cv::Mat imageMat = cv::imread(image);
  stringstream str;
  for(int y = 0; y < imageMat.size().height; y++) {
    for(int x = 0; x < imageMat.size().width; x++) {
      str << "PX " << dec << x+xOffset << " " << y+yOffset << " "
          << b2h(imageMat.at<cv::Vec3b>(y, x)[0])
          << b2h(imageMat.at<cv::Vec3b>(y, x)[1])
          << b2h(imageMat.at<cv::Vec3b>(y, x)[2])
          << "\n";
    }
  }

  command = str.str();
}

Floodgate::~Floodgate() {
  shut();
}

bool Floodgate::open() {
  socketFd = socket(AF_INET, SOCK_STREAM, 0);
  if(socketFd == -1) {
    return false;
  }

  server.sin_addr.s_addr = inet_addr(host.c_str());
  server.sin_family = AF_INET;
  server.sin_port = htons(port);

  if(connect(socketFd, (struct sockaddr*) &server, sizeof(struct sockaddr_in))) {
    return false;
  }

  return true;
}

void Floodgate::flood() {
  const char* data = command.data();
  size_t len = command.length();
  size_t offset = 0;

  while(true) {
    size_t written = write(socketFd, data+offset, len-offset);
    offset += written;

    if(written < 0 || offset >= len) {
      offset = 0;
    }
  }
}


void Floodgate::shut() {
  close(socketFd);
}

string Floodgate::b2h(uint8_t value) {
  char buf[3];
  sprintf(buf, "%02X", value);
  return string(buf);
}
