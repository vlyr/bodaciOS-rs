mkdir build/iso/boot/grub -p

# Compilation and linking
cargo build --release
nasm -felf64 src/asm/multiboot_header.S -o build/multiboot_header.o
nasm -felf64 src/asm/start.S -o build/start.o
x86_64-elf-ld -n -o kernel.bin -T linker.ld build/multiboot_header.o build/start.o target/x86_64-none-bare_metal/release/librust_bodaci.a

# Generate ISO file
cp grub.cfg build/iso/boot/grub
mv kernel.bin build/iso/boot

grub-mkrescue -o os.iso build/iso

rm build -r
qemu-system-x86_64 os.iso
