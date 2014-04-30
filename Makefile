CC = gcc
CFLAGS = -Wall -Werror -O3 -g

all: primes

primes:
	$(CC) $(CFLAGS) primes.cpp -o primes
