# rustc target
TARGET = thumbv7m-none-eabi

# toolchain prefix
TRIPLE = arm-none-eabi

APP_DIR = src/app
OUT_DIR = target/$(TARGET)/release

DEPS_DIR = $(OUT_DIR)/deps

BINS = $(OUT_DIR)/%.bin
ELFS = $(OUT_DIR)/%.elf
OBJECTS = $(OUT_DIR)/intermediate/%.o
SOURCES = $(APP_DIR)/%.rs

APPS = $(patsubst $(SOURCES),$(BINS),$(wildcard $(APP_DIR)/*.rs))

RUSTC_FLAGS := -C lto -g $(RUSTC_FLAGS)

# don't delete my elf files!
.SECONDARY:

all: rlibs  $(APPS)

clean:
	cargo clean

# TODO $(APPS) should get recompiled when the `rlibs` change
$(OBJECTS): $(SOURCES)
	mkdir -p $(dir $@)
	rustc \
		$(RUSTC_FLAGS) \
		--crate-type staticlib \
		--emit obj \
		--target $(TARGET) \
		-L $(DEPS_DIR) \
		-o $@ \
		$<

$(ELFS): $(OBJECTS)
	$(TRIPLE)-ld \
	--gc-sections \
	-T layout.ld \
	-o $@ \
	$<
	size $@

$(BINS): $(ELFS)
	$(TRIPLE)-objcopy \
		-O binary \
		$< \
		$@

rlibs:
	cargo build --target $(TARGET) --release
