all: build run-debug
run: open

build:
	cargo build

build-skylake:
	RUSTFLAGS="-Ctarget-cpu=skylake" cargo build --release

run-debug:
	./target/debug/raytracer

open:
	feh --auto-zoom --force-aliasing out/image.ppm

clean:
	rm -rf target

.PHONY: build build-skylake run-debug open clean
