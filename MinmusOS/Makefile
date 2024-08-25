# Project Name:  MinmusOS
# File Name:     Makefile
# File Function: Automated process build script
# Author:        Jishen Lin
# License:       MIT License

MCOPY := mcopy
OBJCOPY := objcopy
DISK_LAYOUT := "label: dos\n\
				label-id: 0xfffb00b5\n\
				device: disk.img\n\
				unit: sectors\n\
				sector-size: 512\n\
				disk.img1: start=2048, size=2048, type=0\n\
				disk.img2: start=4096, size=32768, type=0\n\
				disk.img3: start=36864, size=94208, type=6"
APPS := $(notdir $(patsubst %/,%,$(shell find apps -mindepth 1 -maxdepth 1 -type d)))
FILES := $(notdir $(wildcard files/*))

.PHONY: run
run: all
	@echo "[INFO] Running system simulation with QEMU..."
	@echo "[INFO] Press CTRL+C to terminate the simulation at any time."
	@qemu-system-i386 -drive file=build/disk.img,index=0,media=disk,format=raw,if=ide

.PHONY: clean
clean:
	@echo "[INFO] Cleaning up..."
	@cargo clean
	@rm -rf build
	@echo "[INFO] Cleanup complete."

.PHONY: build
build:
	@echo "[INFO] Building boot, bootloader, and kernel..."
	@cargo build --target=x86_16-minmus.json --package=boot
	@cargo build --target=x86_16-minmus.json --package=bootloader
	@cargo build --target=x86_32-minmus.json --package=kernel
	@echo "[INFO] Building applications..."
	$(foreach app,$(APPS),cargo build --target=x86_32-minmus.json --package=$(app);)

.PHONY: objcopy
objcopy:
	@echo "[INFO] Copying ELF binaries to binary format..."
	@mkdir -p build/apps
	@$(OBJCOPY) -I elf32-i386 -O binary target/x86_16-minmus/debug/boot build/boot.bin
	@$(OBJCOPY) -I elf32-i386 -O binary target/x86_16-minmus/debug/bootloader build/bootloader.bin
	@$(OBJCOPY) -I elf32-i386 -O binary target/x86_32-minmus/debug/kernel build/kernel.bin
	$(foreach app,$(APPS),$(OBJCOPY) -I elf32-i386 -O binary target/x86_32-minmus/debug/$(app) build/apps/$(app).bin;)

.PHONY: image
image:
	@echo "[INFO] Creating disk image..."
	@dd if=/dev/zero of=build/disk.img bs=67108864 count=1
	@echo "[INFO] Applying disk layout..."
	@echo $(DISK_LAYOUT) | /sbin/sfdisk build/disk.img
	@echo "[INFO] Copying boot sector..."
	@dd if=build/boot.bin of=build/disk.img conv=notrunc
	@echo "[INFO] Preparing partition..."
	@dd if=build/disk.img of=build/partition.img bs=512 skip=36864
	@echo "[INFO] Formatting partition with FAT16..."
	@mkfs.fat -F 16 build/partition.img
	@echo "[INFO] Copying files to partition..."
	$(foreach file,$(FILES),$(MCOPY) -i build/partition.img files/$(file) "::$(file)";)
	$(foreach app,$(APPS),$(MCOPY) -i build/partition.img build/apps/$(app).bin "::$(app)";)
	@echo "[INFO] Finalizing disk image..."
	@dd if=build/partition.img of=build/disk.img bs=512 seek=36864 conv=notrunc
	@rm -rf build/partition.img
	@echo "[INFO] Copying bootloader and kernel to disk image..."
	@dd if=build/bootloader.bin of=build/disk.img bs=512 seek=2048 conv=notrunc
	@dd if=build/kernel.bin of=build/disk.img bs=512 seek=4096 conv=notrunc

.PHONY: all
all: build objcopy image
	@echo "[INFO] Image creation completed."