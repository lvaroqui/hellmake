#include <dlfcn.h>
#include <fcntl.h>

#include <cstdio>
#include <fstream>
#include <iostream>

int main() {
  std::ofstream myfile;
  myfile.open("example.txt");
  return 0;
}
