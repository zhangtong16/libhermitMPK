arch ?= x86_64
target ?= $(arch)-unknown-hermit
release ?= 0

opt :=
rdir := debug

ifeq ($(release), 1)
opt := --release
rdir := release
endif

RN :=
ifdef COMSPEC
RM := del
else
RM := rm -rf
endif

.PHONY: default clean

default:
	RUSTFLAGS="-L ../target/$(target)-kernel/$(rdir) -C link-arg=-Tsrc/linker.ld -Ccodegen-units=1 -Cforce-frame-pointers=yes" cargo build $(opt) --target $(target)
	@objcopy --only-keep-debug target/$(arch)-unknown-hermit/$(rdir)/rusty_tests target/$(arch)-unknown-hermit/$(rdir)/rusty_tests.sym
	@objcopy --strip-debug target/$(arch)-unknown-hermit/$(rdir)/rusty_tests

clean:
	@$(RM) target/x86_64-unknown-hermit
