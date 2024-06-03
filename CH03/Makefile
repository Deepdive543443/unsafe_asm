TARGET   = asm.elf
OBJDIR   = obj

ASMFILES = $(wildcard *.s)
CFILES   = $(wildcard *.c)
CPPFILES = $(wildcard *.cpp)
OBJFILES = $(patsubst %, $(OBJDIR)/%, $(CFILES:.c=.o) $(CPPFILES:.cpp=.o) $(ASMFILES:.s=.o))

# Builds
$(shell mkdir -p $(OBJDIR))

$(TARGET) : $(OBJFILES)
	g++ $(OBJFILES) -o $@

$(OBJDIR)/%.o: %.cpp
	g++ -c $< -o $@

$(OBJDIR)/%.o: %.c
	gcc -c $< -o $@

$(OBJDIR)/%.o: %.s
	gcc -c $< -o $@

.PHONY:	clean

clean:
	rm -f $(TARGET)
	rm -rf $(OBJDIR)