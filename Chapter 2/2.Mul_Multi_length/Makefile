TARGET = asm.elf
OBJDIR = obj
LIBDIR = ../lib/out

CFILES   = $(wildcard *.c)
CPPFILES = $(wildcard *.cpp)
OBJFILES = $(patsubst %, $(OBJDIR)/%, $(CFILES:.c=.o) $(CPPFILES:.cpp=.o))

# Builds
$(shell mkdir -p $(OBJDIR))

$(TARGET) : $(OBJFILES)
	g++ $(OBJFILES) -L$(LIBDIR) -lasm_impl -I$(LIBDIR) -o $@

$(OBJDIR)/%.o: %.cpp
	g++ -c $< -L$(LIBDIR) -lasm_impl -I$(LIBDIR) -o $@

$(OBJDIR)/%.o: %.c
	gcc -c $< -L$(LIBDIR) -lasm_impl -I$(LIBDIR) -o $@

.PHONY:	clean

clean:
	rm -f $(TARGET)
	rm -rf $(OBJDIR)