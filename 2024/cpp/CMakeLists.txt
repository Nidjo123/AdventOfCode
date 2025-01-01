cmake_minimum_required(VERSION 3.30)
project(cpp)

set(CMAKE_CXX_STANDARD 23)

add_executable(cpp Main.cpp
        util/Util.cpp
        util/Util.h
        days/Day.cpp
        days/Day.h
        days/Day01.cpp
        days/Day01.h
        days/Day02.cpp
        days/Day02.h
        days/Day03.cpp
        days/Day03.h
        days/Day04.cpp
        days/Day04.h
        days/Day05.cpp
        days/Day05.h
        days/Day06.cpp
        days/Day06.h
        days/Day07.cpp
        days/Day07.h)

target_include_directories(cpp PRIVATE days util)


file(COPY inputs DESTINATION .)

