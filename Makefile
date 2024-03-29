all: csrc/libao_fec.a cexamples/demo
	cd csrc && make
	cd cexamples && make

clean:
	cd csrc && make clean
	cd cexamples && make clean
