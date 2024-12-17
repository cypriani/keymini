# Instructions

Choose a version, go to the corresponding directory.

Go to [JLCPCB](https://jlcpcb.com/). Upload the zip file as "gerber
file". Choose the PCB color and surface finish. The other options
should be OK as default.

Activate PCB Assembly. Assemble top side, economic. Select "tooling
holes: added by JLCPCB. The other options should be OK as default.

Add "boom.csv" as BOM file and "keymini-top-pos.csv" as CLP file.

Check that the components are found and in stock. The USB-C connector
has a standard footprint, so another reference can be used. Also, you
can use a STM32F072C8T6 if the CB variant is not available (less
flash, but enough for a keyboard).

Check the position of the USB-C connector. Check the direction of the
diodes (the bar should be on the left). Check that the LDO (the 3 pin
chip) is correctly placed. Check that the MCU is correctly placed,
with the marked pin aligned with the mark on the silkscreen (bottom
left).

2024-11-04, I payed 63.62€ including tax and shipping to France using
"Global Standard Direct Line".

# Change log

## v0.2 (TBD)

* add copper pur on the top
* remove SWD connector

## v0.1 (2024-11-04)

Manufactured SMT.

Sent to manufacturing at JLCPLB. Fully functionnal. The PCB are
warped, maybe because of the ground filling only on one side. This
warping makes an anoying balance of the keyboard when pushing on the
thumb keys. A workaround is to insert a spacer on the back between the
silicone sheet and the PCB under the BOOT connector.
