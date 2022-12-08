.PHONY: run
run: NoSpaceLeftOnDevice.java example.txt data.txt
	java NoSpaceLeftOnDevice.java example.txt
	java NoSpaceLeftOnDevice.java data.txt

.PHONY: test
test: NoSpaceLeftOnDevice.java example.txt example.out data.txt data.out
	java NoSpaceLeftOnDevice.java example.txt | diff - example.out
	java NoSpaceLeftOnDevice.java data.txt | diff - data.out
