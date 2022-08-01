TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
# $(wildcard *.xx)
# 匹配目录下所有满足该后缀的文件，包括深层目录中的文件
APPS := $(wildcard $(APP_DIR)/*.rs)
# $(patsubst 原模式, 目标模式, 文件列表)
# 将文件中的匹配原模式的替换成目标模式后缀
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))
BINS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%.bin, $(APPS))

OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

elf: $(APPS)
	@python3 build.py
	

# $(foreach var, list, command)
# var 局部变量
# list 文件列表，空格隔开 每一次取一个值赋值为变量var
# command  对var变量进行操作 每次操作结果都会以空格隔开，最后返回空格隔开的列表。
binary: elf
	$(foreach elf, $(ELFS), $(OBJCOPY) $(elf) --strip-all -O binary $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf));)

build: binary

clean:
	@cargo clean

.PHONY: elf binary build clean