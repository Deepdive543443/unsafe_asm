TARGET = c6
SRCDIR = src
OBJDIR = obj
INCDIR = include
OUTDIR = out


ASMFILES = $(shell ls $(SRCDIR))
OBJFILES = $(OBJDIR)/$(ASMFILES:.s=.o)
ASMFLAGS = -march=armv8-a+simd

$(shell mkdir -p $(OBJDIR))

$(TARGET) : $(OBJFILES)
	ar -rcs $(OBJDIR)/lib$@.a $<

$(OBJDIR)/%.o: $(SRCDIR)/%.s
	as $(ASMFLAGS) $< -o $@

.PHONY:	clean

.PRECIOUS: obj/%.s

install:
	$(shell mkdir -p $(OUTDIR))
	cp $(OBJDIR)/lib$(TARGET).a $(OUTDIR)
	cp $(INCDIR)/*.h $(OUTDIR)

debug:
	@echo $(ASMFILES)
	@echo $(OBJFILES)

clean:
	rm -f $(TARGET)
	rm -rf $(OBJDIR)
	rm -rf $(OUTDIR)