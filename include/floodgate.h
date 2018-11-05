#ifndef FLOODGATE_HPP
#define FLOODGATE_HPP

#include <string>
#include <sys/socket.h>
#include <sys/types.h>
#include <arpa/inet.h>
#include <opencv2/opencv.hpp>

using namespace std;

class Floodgate {
    public:
        Floodgate(string image, string host, uint16_t port, int x_offset, int y_offset);
        ~Floodgate();

        bool open();
        void flood();
        void shut();

    private:
        const string host;
        uint16_t port;
        int sockfd;
        struct sockaddr_in server;
        string command;

        string b2h(uint8_t value);
};

#endif // FLOODGATE_HPP
