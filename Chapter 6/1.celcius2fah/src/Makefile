TARGET   = asm.elf
OBJDIR   = obj
LIBDIR   = ../../lib/out

ASMFILES = $(wildcard *.s)
CFILES   = $(wildcard *.c)
CPPFILES = $(wildcard *.cpp)
OBJFILES = $(patsubst %, $(OBJDIR)/%, $(CFILES:.c=.o) $(CPPFILES:.cpp=.o) $(ASMFILES:.s=.o))

ASMFLAGS = -march=armv8-a+simd
CFLAGS   = $(ASMFLAGS) -O3 -Wall
CPPFLAGS = $(CFLAGS) -std=c++11
LINKFLAG = -I$(LIBDIR) -L$(LIBDIR) -lc6

# Builds
$(shell mkdir -p $(OBJDIR))

$(TARGET) : $(OBJFILES)
	g++ $(CPPFLAGS) $(OBJFILES) $(LINKFLAG) -o $@

$(OBJDIR)/%.o: %.s
	as $(ASMFLAGS) $< -o $@

$(OBJDIR)/%.o: $(OBJDIR)/%.s
	as $(ASMFLAGS) $< -o $@

$(OBJDIR)/%.s: %.cpp
	g++ $(CPPFLAGS) $(LINKFLAG) -S $< -o $(OBJDIR)/$(<:.cpp=.s)

$(OBJDIR)/%.s: %.c
	gcc $(CFLAGS) $(LINKFLAG) -S $< -o $(OBJDIR)/$(<:.c=.s)

.PHONY:	clean

.PRECIOUS: obj/%.s

clean:
	rm -f $(TARGET)
	rm -rf $(OBJDIR)