all: gradient-tool

.PHONY: all clean FORCE

target/release/gradient-tool: FORCE
	cargo build --release

# Non-existent target used to require a target be run if it has
# 'FORCE' as a dependency. Used to avoid having to specify exact dependencies
# on target/release/gradient-tool -- cargo(1) does dependency calculation
# *and* dependency-update checking on the relevant code anyway.
#
# See <https://www.gnu.org/software/make/manual/html_node/Force-Targets.html>.
FORCE:

gradient-tool: target/release/gradient-tool
	cp $< $@
	strip $@

clean:
	rm -rf gradient-tool target/
