default:
	@echo "Usage: make <file>"

%: %.rs
	rustc $< -o build/$@
	./build/$@

clean:
	rm -f build/
