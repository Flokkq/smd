cmake_minimum_required(VERSION 3.25)
project(smd C)

set(CMAKE_C_STANDARD 11)

include_directories(lib)

add_executable(smd
        lib/smd.c
        lib/smd.h
        main.c lib/nec.h lib/nec.c)
