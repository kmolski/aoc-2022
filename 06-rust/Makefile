.PHONY: run
run: TuningTrouble example.txt data.txt
	./TuningTrouble example.txt
	./TuningTrouble data.txt

.PHONY: test
test: TuningTrouble example.txt example.out data.txt data.out
	./TuningTrouble example.txt | diff - example.out
	./TuningTrouble data.txt | diff - data.out

TuningTrouble: TuningTrouble.rs
	rustc TuningTrouble.rs
