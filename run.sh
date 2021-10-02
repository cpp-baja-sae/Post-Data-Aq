#!/bin/bash
black=`tput setaf 0`
red=`tput setaf 1`
green=`tput setaf 2`
yellow=`tput setaf 3`
blue=`tput setaf 4`
magenta=`tput setaf 5`
cyan=`tput setaf 6`
white=`tput setaf 7`
reset_colors=`tput sgr0`

run()
{
    echo "Running script"
    # Run executable

    # Move into proper directory and run the executable
    $(pwd)/build/*.exe "hello world bb"
}

build()
{
    echo "Building files"

    # RUNNING CMAKE
    # Navigate into build directory
    
    cd build/
    # Generate relevant Makefile's in current directory 
    # This is sets a specific file generator "compiler" for 
    # everyone to use.
    cmake -G "MinGW Makefiles" ..
   
    # Build executable - located in build/Debug/
    cmake --build .
}

clean()
{
    # Empty build directory, fresh build
    rm -rf build/*
    # Recover single deleted file
    git restore build/README.md
    echo $red"Contents of build/ have been dumped"$reset_colors
}

debug()
{
    echo "not implemented yet"
}

#########################
# Cmmand line help #
#########################
display_help()
{
    
    echo "[-r | --run]      Run script"
    echo "[-b | --build]    Build script"
    echo "[-d | --debug]    Setup debugger"
    echo "[-c | --clean]    Clean up generated files"
    echo "[-h | --help]     Display avaliable commands"
}
################################
# Check if parameters options  #
# are given on the commandline #
################################
#while getopts r:b:d:h: flag;
# do nothing with options for now
while true ;
do
    case $1 in
        "-r" | "--run")
            run
            break
            ;;
        "-b" | "--build") 
            build
            break
            ;;
        "-d" | "debug") 
            debug
            break
            ;;
        "-h" | "--help")
            display_help
            break
            ;;
        "-c" | "--clean")
            clean
            break
            ;;
    esac
    if [[   $1 -eq ''   ]]
    then
        echo $red"Error - None/Incorrect arguments please refer to"$reset_colors
        display_help
    fi
    break
done




