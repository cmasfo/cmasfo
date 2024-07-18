
cmake_minimum_required(VERSION 3.28)

project(cmake)

include(.cmake/config.cmake)

# ---

add_subdirectory(main)

file(GLOB proj_list proj/cmake/*)

foreach(proj ${proj_list})
  add_subdirectory(${proj})
endforeach()
