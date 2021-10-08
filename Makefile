# *****************************************************
# CURRENTLY MAKEFILE IS DEPERCATED
# PLANNED TO BE USED IN A LATER TIME BUT NOT NOW
# USE FOR UNIT TESTING POSSIBLY
# Variables to control Makefile operation
# Print settings FOR BASH TERMINAL USAGE ONLY
black=`tput setaf 0`
red=`tput setaf 1`
green=`tput setaf 2`
yellow=`tput setaf 3`
blue=`tput setaf 4`
magenta=`tput setaf 5`
cyan=`tput setaf 6`
white=`tput setaf 7`
reset_colors=`tput sgr0`

# Setup some variables
BIN 	:= ./bin
OBJ     := ./obj
INCLUDE := ./include
SRC     := ./src
# Collect files in folders
SRCS    := $(wildcard $(SRC)/*.cpp)
OBJS    := $(patsubst $(SRC)/%.c,$(OBJ)/%.o,$(SRCS))
# bash terminal file delete command
RM=rm -f
# Compiler
CC = g++
# Flags to pass compiler
CFLAGS=-Iinclude/	-Wall -g

# ****************************************************
# Targets needed to bring the executable up to date
# "all" is used by vscode to leverage make
all: main	selfclean

main:	main.o
	$(CC)	$(CFLAGS)	-o	main	*.o

# The main.o target can be written more simply

main.o: $(OBJS)
	@echo	"${green}Source files being compiled ${yellow}"
	@echo	$(OBJS)	${reset_colors}
	$(CC)	$(CFLAGS)	-c	$(OBJS)

clean:
	$(RM)	*.o	*.exe

selfclean:
	@echo "${blue}Removing .o files ${reset_colors}"
	$(RM)	*.o