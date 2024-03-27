CC = gcc
CFLAGS = -I../csrc -Wall -Werror -Wextra -g # -O3
LIBS = ../csrc/libao_fec.a

demo: demo.o $(LIBS)
	$(CC) $(CFLAGS) -o demo demo.o $(LIBS)

$(LIBS): frc
	cd ../csrc && make

clean:
	-rm -f demo demo.o

frc:
