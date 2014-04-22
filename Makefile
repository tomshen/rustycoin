all:
	rustc -L deps/rust-bignum -L deps/rust-opencl/build primes.rs

deps:
	cd deps
	cd rust-bignum
	make build
	cd ../rust-opencl
	make build
	cd ../..

clean:
	rm primes

test:
	rustc --test primes.rs
