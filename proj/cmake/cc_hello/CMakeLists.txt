
get_filename_component(PROJ_NAME ${CMAKE_CURRENT_SOURCE_DIR} NAME)

project(${PROJ_NAME})

add_executable(${PROJ_NAME} src/main.cc)
