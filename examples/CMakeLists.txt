cmake_minimum_required(VERSION 3.10)
project(node)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11 -fPIC -Wall")

find_path(
    HIREDIS_INCLUDE_DIR
    NAMES hiredis/hiredis.h
    PATH_SUFFIXES include
)

find_library(
    HIREDIS_LIBRARIES
    NAMES hiredis
    PATH_SUFFIXES ${CMAKE_INSTALL_LIBDIR}
)

add_executable(node
    main.cpp
)

target_link_libraries(node
    ${HIREDIS_LIBRARIES}
)

target_include_directories(node
PRIVATE
    ${HIREDIS_INCLUDE_DIRS}
)