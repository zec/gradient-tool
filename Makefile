all: gradient-tool

target/release/gradient-tool: FORCE
	cargo build --release

FORCE:

gradient-tool: target/release/gradient-tool
	cp $< $@
	strip $@
