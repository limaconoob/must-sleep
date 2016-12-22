NAME := "nya"
SRCS := lib.rs
DIRC := src/
LSTC := $(patsubst %,$(DIRC)%,$(SRCS))

.SILENT: all
.PHONY: default all

default: all

all:
	rustc --verbose src/lib.rs --crate-name $(NAME) --crate-type dylib --out-dir .
