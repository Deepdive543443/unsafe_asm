TARGET = asm.elf
OBJDIR = obj
LIBDIR = ../lib/out

ASMFILES = $(wildcard *.s)
CFILES   = $(wildcard *.c)
CPPFILES = $(wildcard *.cpp)
OBJFILES = $(patsubst %, $(OBJDIR)/%, $(CFILES:.c=.o) $(CPPFILES:.cpp=.o) $(ASMFILES:.s=.o))

CFLAGS   = -march=armv8-a+simd -O3 -Wall
CPPFLAGS = $(CFLAGS) -std=c++11
ASMFLAGS = -march=armv8-a+simd

LINK_FLAGS = -L$(LIBDIR) -lasm_impl -I$(LIBDIR)

# Builds
$(shell mkdir -p $(OBJDIR))

$(TARGET) : $(OBJFILES)
	g++ $(CPPFLAGS) $(OBJFILES) $(LINK_FLAGS) -o $@

$(OBJDIR)/%.o: %.s
	as $(ASMFLAGS) $< -o $@

$(OBJDIR)/%.o: $(OBJDIR)/%.s
	as $(ASMFLAGS) $< -o $@

$(OBJDIR)/%.s: %.cpp
	g++ $(CPPFLAGS) $(LINK_FLAGS) -S $< -o $(OBJDIR)/$(<:.cpp=.s)

$(OBJDIR)/%.s: %.c
	gcc $(CFLAGS) $(LINK_FLAGS) -S $< -o $(OBJDIR)/$(<:.c=.s)

.PHONY:	clean

.PRECIOUS: obj/%.s

clean:
	rm -f $(TARGET)
	rm -rf $(OBJDIR)