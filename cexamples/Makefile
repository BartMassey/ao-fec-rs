CC = gcc
CFLAGS = -I../csrc -Wall -Werror -g # -O3
LIBS = ../csrc/libao_fec.a

demo: demo.o $(LIBS)
	$(CC) $(CFLAGS) -o demo demo.o $(LIBS)

$(LIBS):
	cd ../csrc && make

clean:
	-rm -f demo demo.o
