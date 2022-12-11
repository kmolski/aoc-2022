.PHONY: run
run: CathodeRayTube.java example.txt data.txt
	java CathodeRayTube.java example.txt
	java CathodeRayTube.java data.txt

.PHONY: test
test: CathodeRayTube.java example.txt example.out data.txt data.out
	java CathodeRayTube.java example.txt | diff - example.out
	java CathodeRayTube.java data.txt | diff - data.out
