.PHONY: run
run: TreetopTreeHouse.java example.txt data.txt
	java TreetopTreeHouse.java example.txt
	java TreetopTreeHouse.java data.txt

.PHONY: test
test: TreetopTreeHouse.java example.txt example.out data.txt data.out
	java TreetopTreeHouse.java example.txt | diff - example.out
	java TreetopTreeHouse.java data.txt | diff - data.out
