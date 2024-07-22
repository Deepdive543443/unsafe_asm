TARGET   = asm.elf
RSTFILES = $(wildcard *.rs)

$(TARGET) : $(RSTFILES)
	rustc $< -o $@

.PHONY:	clean

.PRECIOUS: obj/%.s

clean:
	rm -f *elf
	rm -rf $(OBJDIR)