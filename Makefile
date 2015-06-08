PROJECT=test
TARGET=thumbv7em-none-eabi
OUTDIR=target/$(TARGET)/debug
HEX=$(OUTDIR)/$(PROJECT).hex
ELF=$(OUTDIR)/$(PROJECT)

all:: $(HEX)

$(HEX): $(ELF)
	arm-none-eabi-objcopy -R .stack -O ihex $(ELF) $(HEX)

.PHONY: $(ELF)
$(ELF): 
	cargo build --target $(TARGET)

flash: $(HEX)
	@echo "Reset your Teensy now!"
	teensy-loader-cli -w --mcu=mk20dx256 $(HEX)
