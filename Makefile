build:
	bootimage build --target x86_64-weed.json

run: build
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-weed/debug/bootimage-weed-os.bin
