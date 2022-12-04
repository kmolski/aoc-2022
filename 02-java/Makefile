.PHONY: run
run: RockPaperScissors.java example.txt data.txt
	java RockPaperScissors.java example.txt
	java RockPaperScissors.java data.txt

.PHONY: test
test: RockPaperScissors.java example.txt example.out data.txt data.out
	java RockPaperScissors.java example.txt | diff - example.out
	java RockPaperScissors.java data.txt | diff - data.out
