DEPS ?= $(wildcard deps/*)

.PHONY: cleandeps deps

all:
	rustc -L deps/rust-bignum/build primes.rs

deps:
	@for dep in $(DEPS) ; do \
		$(MAKE) -w -C $$dep deps && $(MAKE) -w -C $$dep && $(MAKE) -w -C $$dep build ; \
	done

clean: cleandeps
	rm -f primes

cleandeps:
	@for dep in $(DEPS) ; do \
		$(MAKE) -w -C $$dep clean ; \
	done

test:
	rustc -L deps/rust-bignum/build --test primes.rs
