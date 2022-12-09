.PHONY: run
run: RopeBridge example.txt data.txt
	./RopeBridge example.txt
	./RopeBridge data.txt

.PHONY: test
test: RopeBridge example.txt example.out data.txt data.out
	./RopeBridge example.txt | diff - example.out
	./RopeBridge data.txt | diff - data.out

RopeBridge: RopeBridge.rs
	rustc --edition 2021 RopeBridge.rs
