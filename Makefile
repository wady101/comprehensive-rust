default:
	@echo "Usage: make <file>"

build/%:
	mkdir -p $(dir $@)


# MANUALLY ADD your test files here:
test-2/expression_evaluation: 2/expression_evaluation.rs | build
	rustc --test 2/expression_evaluation.rs -o build/expression_evaluation
	./build/expression_evaluation

test-2/min: 2/min.rs | build
	rustc --test 2/min.rs -o build/min
	./build/min

test-2/rot13: 2/rot13.rs | build
	rustc --test 2/rot13.rs -o build/rot13
	./build/rot13

clean:
	rm -rf build/
%: %.rs
	rustc $< -o build/$@
	./build/$@


clean:
	rm -rf build/
