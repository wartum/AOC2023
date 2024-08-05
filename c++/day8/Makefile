include ../makedefs.mk

DAY_NO=8
ADD_SRC=
ADD_OBJ=$(ADD_SRC:%.cpp=%.o)
CXXFLAGS+=

day$(DAY_NO): day$(DAY_NO).o $(ADD_OBJ)
	$(CXX) -o day$(DAY_NO) $(CXXFLAGS) $^

clean:
	rm -f day$(DAY_NO) day$(DAY_NO).o $(ADD_OBJ)
