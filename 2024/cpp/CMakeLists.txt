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
        days/Day02.h)

target_include_directories(cpp PRIVATE days util)


file(COPY inputs DESTINATION .)

