DIRS=$(wildcard day*)
export CXX=clang++
export CXXFLAGS=--std=c++20 -O3

.PHONY: all clean

all:
	for dir in $(DIRS); do \
		make -C $$dir $$dir ; \
	done

clean:
	for dir in $(DIRS); do \
		make -C $$dir clean ; \
	done
