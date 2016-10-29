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
	~/.cargo/bin/xargo build --target $(TARGET)

flash: $(HEX)
	@echo "Reset your Teensy now!"
	teensy-loader-cli -w -mmcu=mk20dx128 $(HEX) -v
