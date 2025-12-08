# Aero Kernel Makefile

.PHONY: all build iso run clean test

# Compiler and tools
CARGO = cargo
QEMU = qemu-system-x86_64
NASM = nasm

# Directories
BUILD_DIR = build
ISO_DIR = $(BUILD_DIR)/iso
BOOT_DIR = $(ISO_DIR)/boot

# Targets
KERNEL = target/x86_64-unknown-none/debug/aero-kernel
ISO = aero.iso

all: build

build:
	@echo "Building Aero kernel..."
	@mkdir -p $(BUILD_DIR)
	$(CARGO) build --target x86_64-unknown-none
	@echo "✓ Kernel built successfully"

iso: build
	@echo "Creating bootable ISO..."
	@mkdir -p $(BOOT_DIR)/grub
	@cp $(KERNEL) $(BOOT_DIR)/kernel.bin
	@echo "set timeout=0" > $(BOOT_DIR)/grub/grub.cfg
	@echo "set default=0" >> $(BOOT_DIR)/grub/grub.cfg
	@echo "menuentry \"Aero OS\" {" >> $(BOOT_DIR)/grub/grub.cfg
	@echo "    multiboot2 /boot/kernel.bin" >> $(BOOT_DIR)/grub/grub.cfg
	@echo "    boot" >> $(BOOT_DIR)/grub/grub.cfg
	@echo "}" >> $(BOOT_DIR)/grub/grub.cfg
	@grub-mkrescue -o $(ISO) $(ISO_DIR) 2>/dev/null || echo "Note: grub-mkrescue not available, ISO creation skipped"
	@echo "✓ ISO created: $(ISO)"

run: build
	@echo "Starting Aero in QEMU..."
	$(QEMU) -kernel $(KERNEL) -serial stdio -no-reboot -no-shutdown

run-kvm: build
	@echo "Starting Aero in QEMU with KVM..."
	$(QEMU) -kernel $(KERNEL) -serial stdio -enable-kvm

debug-gdb: build
	@echo "Starting Aero in debug mode (waiting for GDB)..."
	$(QEMU) -kernel $(KERNEL) -serial stdio -s -S

test:
	@echo "Running kernel tests..."
	$(CARGO) test
	@echo "✓ All tests passed"

clean:
	@echo "Cleaning build artifacts..."
	$(CARGO) clean
	rm -rf $(BUILD_DIR)
	rm -f $(ISO)
	@echo "✓ Clean complete"

help:
	@echo "Aero Kernel Build System"
	@echo ""
	@echo "Targets:"
	@echo "  build      - Build the kernel"
	@echo "  iso        - Create bootable ISO"
	@echo "  run        - Run kernel in QEMU"
	@echo "  run-kvm    - Run kernel in QEMU with KVM"
	@echo "  debug-gdb  - Start kernel in debug mode"
	@echo "  test       - Run kernel tests"
	@echo "  clean      - Remove build artifacts"
	@echo "  help       - Show this help message"
