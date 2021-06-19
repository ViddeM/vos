kernel_source_files := $(shell find src/ -name *.rs)
kernel_object_files := $(patsubst src/%.rs, build/kernel/%.o, $(kernel_source_files))

x86_64_asm_source_files := $(shell find src/x86_64 -name *.asm)
x86_64_asm_object_files := $(patsubst src/x86_64/%.asm, build/x86_64/%.o, $(x86_64_asm_source_files))

x86_64_object_files := $(x86_64_asm_object_files)

$(x86_64_asm_object_files): build/x86_64/%.o : src/x86_64/%.asm
	mkdir -p $(dir $@) && \
	nasm -f elf64 $(patsubst build/x86_64/%.o, src/x86_64/%.asm, $@) -o $@

$(kernel_object_files): build/kernel/%.o : src/%.rs
	mkdir -p $(dir $@) && \
	cargo rustc --lib -- --emit=obj -o $@
#	rustc -C target-feature=+crt-static --crate-type=staticlib --emit=obj src/main.rs -o $@
#	rustc -C target-feature=+crt-static --crate-type=staticlib --emit=obj $(patsubst build/kernel/%.o, src/%.rs, $@) -o $@

.PHONY: build-x86_64
build-x86_64: $(kernel_object_files) $(x86_64_object_files)
	mkdir -p dist/x86_64 && \
	x86_64-elf-ld -n -o dist/x86_64/kernel.bin -T arch/x86_64/linker.ld $(kernel_object_files) $(x86_64_object_files) && \
	cp dist/x86_64/kernel.bin arch/x86_64/iso/boot/kernel.bin && \
	grub-mkrescue /usr/lib/grub/i386-pc -o dist/x86_64/kernel.iso arch/x86_64/iso