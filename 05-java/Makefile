.PHONY: run
run: SupplyStacks.java example.txt data.txt
	java SupplyStacks.java example.txt
	java SupplyStacks.java data.txt

.PHONY: test
test: SupplyStacks.java example.txt example.out data.txt data.out
	java SupplyStacks.java example.txt | diff - example.out
	java SupplyStacks.java data.txt | diff - data.out
