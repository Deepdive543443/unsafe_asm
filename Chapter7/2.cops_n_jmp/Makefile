TARGET   = vec_add
LIBNAME  = c7

all: $(TARGET) 

$(TARGET) : lib$(LIBNAME).a
	cargo build --release
	@echo "\033[92mTarget $(TARGET) builded\033[0m"

lib$(LIBNAME).a :
	make -C ../lib
	make -C ../lib install
	@echo "\033[92mTarget lib$(LIBNAME) builded\033[0m"

.PHONY:	clean

.PRECIOUS: obj/%.s

clean: libclean
	cargo clean

libclean:
	make -C ../lib clean