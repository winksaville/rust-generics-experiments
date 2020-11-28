srcs=src/*
target=x86_64-unknown-linux-musl
bin=generics

# Default build to debug, for release pass build=release on command line:
#   make build=release
build := debug
ifeq '$(build)' 'release'
  build-type=--release
else
  build-type=
endif

built-file=target/$(target)/$(build)/$(bin)

$(bin): $(built-file)

$(built-file): $(srcs)
	cargo build $(build-type) --target $(target)
	@if [ '$(build)' = 'release' ]; then strip $(built-file); fi
	cp $(built-file) $(bin)

.PHONY: clean
clean:
	rm -rf $(built-file) $(bin)

.PHONY: cleanall
cleanall: clean
	rm -rf target
