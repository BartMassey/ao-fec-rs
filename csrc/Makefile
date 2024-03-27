CC = gcc
CFLAGS = -Wall -Werror -g # -O3 

OBJS = ao_fec_tx.o ao_fec_rx.o

libao_fec.a: $(OBJS)
	$(AR) crs libao_fec.a $(OBJS)

ao_fec_tx.o: ao_fec.h ao_whiten.h

ao_fec_rx.o: ao_fec.h

make_whiten: make_whiten.c
	$(CC) -o make_whiten make_whiten.c

ao_whiten.h: make_whiten
	./make_whiten >ao_whiten.h

clean:
	-rm -f *.o make_whiten ao_whiten.h libao_fec.a
