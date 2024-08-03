DIRS=$(wildcard day*)
.PHONY: all clean

all:
	for dir in $(DIRS); do \
		make -C $$dir $$dir ; \
	done

clean:
	for dir in $(DIRS); do \
		make -C $$dir clean ; \
	done
