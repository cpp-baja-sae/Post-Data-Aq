# Post-Data-Aq
```diff
+ CMakeList.txt is used to generate executables
+ file is automatically ran by using run.sh
! Action to do, add file content of build to .gitignore file

- DO NOT PUSH CONTENTS OF BUILD/ INTO REPOSITORY!!

# This program is intended to be only ran on a Windows machine

# Python packages to downloand
pyqt5

 
# Things needed to run system

# GCC compiler used
https://www.youtube.com/watch?v=aXF4A5UeSeM
Download gcc, make, g++

# Cmake - Download 
https://cmake.org/download/
Download cmake-3.21.3-windows-x86_64.msi
Add CMake to system user PATH manually
Check that it is downloaded in the git bash terminal by
cmake --version

# make - Download
Download choco first runnning windows powershell as administrator
https://chocolatey.org/install
Afterwards run
choco install make

# To build and run script you only need to run the shell script using git bash terminal
# run command below to see list of all command available

./run.sh -h 

# When system fails to build perform a clean action using run.sh script

+ While programming, you only need to #include "file.h", you do not need to
+ include the path as the CMakeLists.txt file already is aware of it.
+ VScode might throw an error about it, easy way to fix that is to add

"C_Cpp.default.includePath": [
    "include\\main.h"
    ],
+ To your settings.json file inside the project work space, including all header files as needed.