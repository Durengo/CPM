cmake_minimum_required(VERSION 3.13)
project(sample)
set(CMAKE_CXX_STANDARD 14)
set(SOURCE_FILES main.cpp)

add_executable(sample ${SOURCE_FILES})

install(TARGETS sample
    LIBRARY DESTINATION bin
    ARCHIVE DESTINATION bin
    RUNTIME DESTINATION bin)
install(FILES $<TARGET_RUNTIME_DLLS:sample> DESTINATION bin)