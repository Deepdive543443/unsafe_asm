TARGET   = asm.elf
OBJDIR   = obj
LIBDIR   = ../lib/out
LIBNAME  = c2

RSTFILES = $(wildcard *.rs)
LINKFLAG = -l static=$(LIBNAME) -L $(LIBDIR)

$(TARGET) : $(RSTFILES)
	rustc $(LINKFLAG) $< -o $@

.PHONY:	clean

.PRECIOUS: obj/%.s

clean:
	rm -f *elf
	rm -rf $(OBJDIR)