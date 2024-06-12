TARGET   = asm.elf
OBJDIR   = obj

ASMFILES = $(wildcard *.s)
CFILES   = $(wildcard *.c)
CPPFILES = $(wildcard *.cpp)
OBJFILES = $(patsubst %, $(OBJDIR)/%, $(CFILES:.c=.o) $(CPPFILES:.cpp=.o) $(ASMFILES:.s=.o))

CFLAGS   = -march=armv8-a+simd -O3 -Wall
CPPFLAGS = $(CFLAGS) -std=c++11
ASMFLAGS = -march=armv8-a+simd

# Builds
$(shell mkdir -p $(OBJDIR))

$(TARGET) : $(OBJFILES)
	g++ $(CPPFLAGS) $(OBJFILES) -o $@

$(OBJDIR)/%.o: %.s
	as $(ASMFLAGS) $< -o $@

$(OBJDIR)/%.o: $(OBJDIR)/%.s
	as $(ASMFLAGS) $< -o $@

$(OBJDIR)/%.s: %.cpp
	g++ $(CPPFLAGS) -S $< -o $(OBJDIR)/$(<:.cpp=.s)

$(OBJDIR)/%.s: %.c
	gcc $(CFLAGS) -S $< -o $(OBJDIR)/$(<:.c=.s)

.PHONY:	clean

.PRECIOUS: obj/%.s

clean:
	rm -f $(TARGET)
	rm -rf $(OBJDIR)