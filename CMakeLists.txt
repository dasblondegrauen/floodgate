cmake_minimum_required(VERSION 2.8)

project(floodgate)
find_package(OpenCV REQUIRED)

set(FLOODGATE_SRC
    src/main.cpp
    src/floodgate.cpp
    )

set(FLOODGATE_INCLUDE
    include/floodgate.h
    )

INCLUDE_DIRECTORIES(include ${OpenCV_INCLUDE_DIRS})
add_executable(${PROJECT_NAME} ${FLOODGATE_SRC} ${FLOODGATE_INCLUDE})
target_link_libraries(floodgate ${OpenCV_LIBS})
