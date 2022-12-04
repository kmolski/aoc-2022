.PHONY: run
run: RucksackReorganization example.txt data.txt
	./RucksackReorganization example.txt
	./RucksackReorganization data.txt

.PHONY: test
test: RucksackReorganization example.txt example.out data.txt data.out
	./RucksackReorganization example.txt | diff - example.out
	./RucksackReorganization data.txt | diff - data.out

RucksackReorganization: RucksackReorganization.rs
	rustc RucksackReorganization.rs
