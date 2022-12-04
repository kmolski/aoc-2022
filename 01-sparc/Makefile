.PHONY: run
run: a.out example.txt data.txt
	qemu-sparc64 a.out example.txt
	qemu-sparc64 a.out data.txt

.PHONY: test
test: a.out example.txt example.out data.txt data.out
	qemu-sparc64 a.out example.txt | diff - example.out
	qemu-sparc64 a.out data.txt | diff - data.out

a.out: CalorieCounting.s
	sparc64-linux-gnu-gcc-10 -static -fno-pic -mno-stack-bias CalorieCounting.s
