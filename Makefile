PATH := /usr/local/cuda/bin/:${PATH}
LD_LIBRARY_PATH := /usr/local/cuda/lib64/:${LD_LIBRARY_PATH}

CC = g++
LIBS = gmpxx gmp cudart
LDLIBS := -L/usr/local/cuda/lib64/ $(addprefix -l, $(LIBS))

CFLAGS = -Wall -Werror -O3 -m64 --std=c++0x
NVCCFLAGS = -O3 -m64 -arch compute_20

PRIMES = util.h util.cpp primes.h primes.cpp

all: cuda

cuda:
	$(CC) $(CFLAGS) -c util.cpp -o util.o $(LDLIBS)
	nvcc $(NVCCFLAGS) -c primes.cu -o primes.o
	$(CC) $(CFLAGS) $(LDLIBS) tests.cpp -o cuda-test util.h primes.h util.o primes.o

cpu:
	$(CC) $(CFLAGS) $(LDFLAGS) $(PRIMES) tests.cpp -o cpu-test

clean:
	rm -f *.o test
