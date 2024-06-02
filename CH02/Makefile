TARGET = asm_exe.elf
OBJDIR = obj

CFILES   = $(wildcard *.c)
ASMFILES = $(wildcard *.s)
OBJS     = $(CFILES) $(ASMFILES)

# Builds
$(TARGET) : $(OBJS)
	gcc $(OBJS) -o $@

.PHONY:	clean debug

clean:
	rm -f $(TARGET)
	rm -rf $(OBJDIR)