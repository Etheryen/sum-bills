.PHONY: main clean

CC = clang
CFLAGS = -std=c99 -O3 -Wall -Wextra -Wpedantic -fsanitize=address,undefined,leak

all: main

main: main.c
	$(CC) $(CFLAGS) -o main main.c

clean:
	rm -f ./main
