#ifndef UTIL_H
#define UTIL_H

#include <filesystem>
#include <vector>
#include <string>
#include <iostream>

std::vector<std::string> LoadInputLines(std::filesystem::path path);

template<typename T>
void PrintIterable(const T &iterable) {
    bool first = true;
    for (const auto x: iterable) {
        if (!first) {
            std::cout << ", ";
        } else {
            first = false;
        }
        std::cout << x;
    }
}

#endif //UTIL_H
