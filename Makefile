CC = g++
CFLAGS = -Wall -Werror -O3 -g -std=c++0x -lgmpxx -lgmp

PRIMES = util.h util.cpp primes.h primes.cpp

all:
	$(CC) $(CFLAGS) $(PRIMES) tests.cpp -o test
