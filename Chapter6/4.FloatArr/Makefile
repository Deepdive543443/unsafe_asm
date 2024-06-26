TARGET   = asm.elf
OBJDIR   = obj
LIBDIR   = ../lib/out
LIBNAME  = c6

RSTFILES = $(wildcard *.rs)
LINKFLAG = -l static=$(LIBNAME) -L $(LIBDIR)

$(TARGET) : $(RSTFILES) lib$(LIBNAME).a
	rustc $(LINKFLAG) $< -o $@

lib$(LIBNAME).a :
	make -C ../lib clean
	make -C ../lib
	make -C ../lib install

.PHONY:	clean

.PRECIOUS: obj/%.s

clean:
	rm -f *elf
	rm -rf $(OBJDIR)