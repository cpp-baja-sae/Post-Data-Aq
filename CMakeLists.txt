# Specify the minimum version for CMake
cmake_minimum_required(VERSION 3.21)
#Project name
Project(PostDataAq)
#enable C++17 standards
set(CMAKE_CXX_STANDARD 17)
# Grab include folder and src folder content
include_directories(SYSTEM ${PROJECT_SOURCE_DIR}/include
		SYSTEM ${PROJECT_SOURCE_DIR}/src
)
# Grab names
file(GLOB all_SRCS
        "${PROJECT_SOURCE_DIR}/include/*.h"
		"${PROJECT_SOURCE_DIR}/include/*.hpp"
        "${PROJECT_SOURCE_DIR}/src/*.cpp"
		"${PROJECT_SOURCE_DIR}/src/*.c"
        )
# Setting up compiler warning level 4
# Using -Wall shows a lot of system library related issues
# To many to actually see useful warnings actually made by user

#look into compile flags -W
set_source_files_properties(
  ${all_SRCS}
  PROPERTIES
  COMPILE_FLAGS "-W"
)

add_executable(PostDataAq ${all_SRCS})