default: build

build: target/vos.bin

.PHONY: default build run clean

asm_files := $(shell find src/asm -name *.asm)
asm_object_files := $(patsubst src/asm/%.asm, target/%.o, $(asm_files))

$(asm_object_files): target/%.o : src/asm/%.asm
	mkdir -p target
	nasm -f elf64 $(patsubst target/%.o, src/asm/%.asm, $@) -o $@

target/vos.bin: $(asm_object_files) src/asm/linker.ld cargo
	ld -n -o target/vos.bin -T src/asm/linker.ld $(asm_object_files) target/x86_64-target-vos/release/libvos.a

target/vos.iso: target/vos.bin src/asm/grub.cfg
	mkdir -p target/isofiles/boot/grub
	cp src/asm/grub.cfg target/isofiles/boot/grub
	cp target/vos.bin target/isofiles/boot/
	grub-mkrescue -o target/vos.iso target/isofiles

run: target/vos.iso
	qemu-system-x86_64 -cdrom target/vos.iso

clean:
	cargo clean

iso: target/vos.iso

cargo:
	cargo build --release