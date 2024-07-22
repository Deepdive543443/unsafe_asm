TARGET = c7
SRCDIR = src
INCDIR = include
OBJDIR = obj
OUTDIR = out

ASMFILES = $(shell ls $(SRCDIR))
OBJFILES = $(patsubst %, $(OBJDIR)/%, $(ASMFILES:.s=.o))
ASMFLAGS = -march=armv8-a+simd

# Builds
$(shell mkdir -p $(OBJDIR))
$(TARGET) : $(OBJFILES)
	ar -rcs $(OBJDIR)/lib$@.a $(OBJFILES)
	
$(OBJDIR)/%.o: $(SRCDIR)/%.s
	as $(ASMFLAGS) $< -o $@

.PHONY: clean

install:
	$(shell mkdir -p $(OUTDIR))
	cp $(OBJDIR)/lib$(TARGET).a $(OUTDIR)
	cp $(INCDIR)/*.h $(OUTDIR)

clean:
	rm -f $(TARGET)
	rm -rf $(OBJDIR)
	rm -rf $(OUTDIR)