CC = g++
CFLAGS = -Wall -Werror -O3 -g -std=c++0x -lgmpxx -lgmp

all:
	$(CC) $(CFLAGS) primes.cpp -o primes
