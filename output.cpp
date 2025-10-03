#include <chrono>
#include <windows.h>
#include <thread>
#include <iostream>

int main() {

    std::cout << "Creating windows popup..." << std::endl;
    while(true) {
        MessageBox(nullptr,"This is an example of a generated malware executable running!","Malware Analysis Language", MB_OK);
        Sleep(10 * 1000);
    }

    return 0;
}