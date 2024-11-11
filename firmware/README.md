# Keymini rust firmware

Install the complete toolchain and utils:

```shell
curl https://sh.rustup.rs -sSf | sh
rustup target add thumbv6m-none-eabi
rustup component add llvm-tools-preview
cargo install cargo-binutils
sudo apt-get install dfu-util
```

Compile:

```shell
cd firmware
cargo objcopy --bin keymini --release -- -O binary firmware.bin
```

To flash using dfu-util, launch it with:
```shell
dfu-util -w -d 0483:df11 -a 0 -s 0x08000000:leave -D firmware.bin
```

Then, put the board in DFU mode by:
 * making the bootloader combo as defined in the firmware,
 * or (for the first time, or in case of buggy firmware) by shorting
   the BOOT1 2 pad under the board (with an aluminium foil for
   example) and plug USB.

The upload should began as soon as the computer detect the board in
DFU mode.

The first time, if the write fail, your flash might be protected. To
unprotect:

```shell
dfu-util -d 0483:df11 -a 0 -s 0x08000000:force:unprotect -D firmware.bin
```
