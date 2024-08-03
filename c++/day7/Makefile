include ../makedefs.mk

DAY_NO=7
ADD_SRC=hand.cpp
ADD_OBJ=$(ADD_SRC:%.cpp=%.o)

day$(DAY_NO): day$(DAY_NO).o $(ADD_OBJ)
	$(CXX) -o day$(DAY_NO) $(CXXFLAGS) $^

clean:
	rm -f day$(DAY_NO) day$(DAY_NO).o $(ADD_OBJ)
