IMAGE_NAME=karlos
BUILD_DIR=$(CURDIR)/build

ifeq ($(OS), Windows_NT)
SRC_DIR := $(shell cygpath -m $(CURDIR))
else
SRC_DIR := $(CURDIR)
endif

docker-build:
	echo "$(SRC_DIR)"
	docker build -t $(IMAGE_NAME) .

build:
	@echo "INFO: Comment line 16 if you are not using MSYS2, or else error could occur\n"
	MSYS_NO_PATHCONV=0 \
	docker run --rm \
		-v $(SRC_DIR):/src \
		-w /src \
		$(IMAGE_NAME) \
		make native-build

add-rust-target:
	rustup target add x86_64-unknown-uefi x86_64-unknown-none

add-rust-clippy:
	rustup component add clippy

fmt:
	cargo fmt -p boot
	cargo fmt -p kernel

native-build: add-rust-target
	mkdir iso/EFI/BOOT -p

	cargo build -p boot -r --target x86_64-unknown-uefi
	strip target/x86_64-unknown-uefi/release/boot.efi --strip-all
	cp target/x86_64-unknown-uefi/release/boot.efi iso/EFI/BOOT/BOOTX64.EFI

	RUSTFLAGS="-C link-arg=-Tkernel/linker.ld -C link-arg=-no-pie -C relocation-model=static" \
	cargo build -p kernel -r --target x86_64-unknown-none
	cd target/x86_64-unknown-none/release && \
	objcopy kernel --strip-all  \
		--remove-section .eh_frame \
		--remove-section .eh_frame_hdr \
		--remove-section .got \
		--remove-section .comment \
		--strip-section
	cp target/x86_64-unknown-none/release/kernel iso/KERNEL.ELF

	@echo Please note that directory "iso" was created, it contain the binaries
	@echo If you are cross-compiling on your own, you MUST disable "Secure Boot" or sign all the binaries

build-iso:
	MSYS_NO_PATHCONV=0 \
	docker run --rm \
		-v $(SRC_DIR):/src \
		-w /src \
		$(IMAGE_NAME) \
		make native-build-iso

native-build-iso:
	xorriso -as mkisofs \
		-iso-level 3 \
		-V "UEFI_ONLY" \
		-o karlos.iso \
		-r -J -joliet-long -full-iso9660-filenames \
		-eltorito-alt-boot \
		-e EFI/BOOT/BOOTX64.EFI \
		-no-emul-boot \
		-isohybrid-gpt-basdat \
		iso

sign-bl:
	MSYS_NO_PATHCONV=0 \
	docker run --rm \
		-v $(SRC_DIR):/src \
		-w /src \
		$(IMAGE_NAME) \
		make native-sign-bl

native-sign-bl:
	@echo "Warning: You must change the native-sign args in Makefile to use feature \"native-sign/sign\"!"
	@echo ""
	@echo "INFO: Signing UEFI bootloader..."

	sbsign iso/EFI/BOOT/BOOTX64.EFI \
		--key signature/uefi/karlos.key \
		--cert signature/uefi/karlos.crt --output iso/EFI/BOOT/BOOTX64.EFI

native-run:
	# Edit the args if you want!

	# Comment line 16 if you are not using MSYS2, or else error could occur
	qemu-system-x86_64 \
	-m 512M \
	-bios /usr/share/OVMF/OVMF_CODE.fd \
	-drive format=raw,file=fat:rw:iso

iso-info:
	# Comment next line if you are not using MSYS2
	MSYS_NO_PATHCONV=0 \
	docker run --rm \
		-v $(SRC_DIR):/src \
		-w /src \
		$(IMAGE_NAME) \
		make native-iso-info

native-iso-info:
	xorriso -indev karlos.iso -ls /

docker-clippy: add-rust-target add-rust-clippy
	MSYS_NO_PATHCONV=0 \
	docker run --rm \
		-v $(SRC_DIR):/src \
		-w /src \
		$(IMAGE_NAME) \
		make clippy

clippy:
	@echo "**Start of UEFI bootloader clippy**"
	cargo clippy -p boot --target x86_64-unknown-uefi
	@echo "**End of UEFI bootloader clippy**"
	@echo ""
	@echo ""
	@echo ""
	@echo "**Start of kernel clippy**"
	cargo clippy -p kernel --target x86_64-unknown-none
	@echo "**End of kernel clippy**"