.PHONY: prepare build clean all iso link

prepare:
	mkdir build/iso/boot/grub -p

build: prepare
	nasm -felf64 src/asm/multiboot_header.S -o build/multiboot_header.o
	nasm -felf64 src/asm/boot.S -o build/boot.o
	nasm -felf64 src/asm/long_mode_init.S -o build/long_mode_init.o
	cargo build --release

link: build
	x86_64-elf-ld -n -o kernel.bin -T linker.ld build/long_mode_init.o build/multiboot_header.o build/boot.o target/x86_64-none-bare_metal/release/librust_bodaci.a

iso: link
	cp grub.cfg build/iso/boot/grub
	mv kernel.bin build/iso/boot
	grub-mkrescue -o os.iso build/iso

clean: 
	rm build -r

run: iso
	qemu-system-x86_64 os.iso
