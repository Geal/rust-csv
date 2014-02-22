build:
	mkdir -p build
	rustc --out-dir build -O src/csv/lib.rs

test: build
	rustc -L build -o build/test --test src/csv/test.rs
	./build/test

clean:
	rm -rf build
