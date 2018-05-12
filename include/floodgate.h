#include <string>
#include <sys/socket.h>
#include <sys/types.h>
#include <arpa/inet.h>
#include <opencv2/opencv.hpp>

using namespace std;

class Floodgate {
public:
  Floodgate(string image, string host, uint16_t port);
  ~Floodgate();

  bool open();
  void flood();
  void shut();

private:
  string image, host;
  uint16_t port;
  int socketFd;
  struct sockaddr_in server;
  string command;

  string b2h(uint8_t);
};
