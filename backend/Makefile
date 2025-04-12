CXX = g++
CXXFLAGS = -std=c++17 -O2
LDFLAGS = -lboost_system -ljsoncpp -lpthread -lboost_program_options

all: calendar_boost

calendar_boost: main.o
	$(CXX) -o $@ $^ $(LDFLAGS)

%.o: %.cpp
	$(CXX) -c -o $@ $< $(CXXFLAGS)

clean:
	rm -f *.o calendar_boost
