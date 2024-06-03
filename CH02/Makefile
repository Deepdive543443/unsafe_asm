TARGET   = asm.elf
OBJDIR   = obj

ASMFILES = $(wildcard *.s)
CFILES   = $(wildcard *.c)
OBJFILES = $(patsubst %, $(OBJDIR)/%, $(CFILES:.c=.o) $(ASMFILES:.s=.o))

# Builds
$(shell mkdir -p $(OBJDIR))

$(TARGET) : $(OBJFILES)
	gcc $(OBJFILES) -o $@

$(OBJDIR)/%.o: %.c
	gcc -c $< -o $@

$(OBJDIR)/%.o: %.s
	gcc -c $< -o $@

.PHONY:	clean

clean:
	rm -f $(TARGET)
	rm -rf $(OBJDIR)