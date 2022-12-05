.PHONY: run
run: CampCleanup.java example.txt data.txt
	java CampCleanup.java example.txt
	java CampCleanup.java data.txt

.PHONY: test
test: CampCleanup.java example.txt example.out data.txt data.out
	java CampCleanup.java example.txt | diff - example.out
	java CampCleanup.java data.txt | diff - data.out
