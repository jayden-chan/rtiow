all: build run-debug
run: run-debug open

build:
	cargo build

build-skylake:
	RUSTFLAGS="-Ctarget-cpu=skylake" cargo build --release

run-debug:
	./target/debug/raytracer

open:
	open out/image.ppm

clean:
	rm -rf target

.PHONY: build build-skylake run-debug open clean
