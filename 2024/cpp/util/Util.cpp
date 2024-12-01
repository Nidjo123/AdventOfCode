//
// Created by nikola on 01.12.24..
//

#include "Util.h"

#include <fstream>

std::vector<std::string> LoadInputLines(const std::filesystem::path path) {
    std::ifstream file{path};
    if (!file) {
        throw std::runtime_error("Could not open file");
    }

    std::vector<std::string> lines;
    std::string line;
    while (std::getline(file, line)) {
        lines.push_back(line);
    }

    return lines;
}
