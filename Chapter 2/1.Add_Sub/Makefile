TARGET = asm.elf
OBJDIR = obj
LIBDIR = ../lib/out

CFILES   = $(wildcard *.c)
OBJFILES = $(patsubst %, $(OBJDIR)/%, $(CFILES:.c=.o) $(ASMFILES:.s=.o))

# Builds
$(shell mkdir -p $(OBJDIR))

$(TARGET) : $(OBJFILES)
	gcc $(OBJFILES) -L$(LIBDIR) -lasm_impl -I$(LIBDIR) -o $@

$(OBJDIR)/%.o: %.c
	gcc -c $< -L$(LIBDIR) -lasm_impl -I$(LIBDIR) -o $@

.PHONY:	clean

clean:
	rm -f $(TARGET)
	rm -rf $(OBJDIR)