cmake_minimum_required(VERSION 2.8)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_EXPORT_COMPILE_COMMANDS 1)
set(CMAKE_BUILD_TYPE RelWithDebugInfo)
add_compile_options(-Wall -Wextra -pedantic)

project(floodgate)
find_package(OpenCV REQUIRED)

set(FLOODGATE_SRC
    src/main.cpp
    src/floodgate.cpp
    )

include_directories(include ${OpenCV_INCLUDE_DIRS})
add_executable(${PROJECT_NAME} ${FLOODGATE_SRC})
target_link_libraries(floodgate ${OpenCV_LIBS})
