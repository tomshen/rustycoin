all:
	rustc primes.rs

clean:
	rm primes

test:
	rustc --test primes.rs
