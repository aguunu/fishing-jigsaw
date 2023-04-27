.PHONY: all clean

MAIN            = main
TARGET_EXEC     = app
VALGRIND_LOGS   = valgrind-out

INC_DIR     = ./include
SRC_DIR     = ./src
BUILD_DIR   = ./build
TEST_DIR    = ./test
LOGS_DIR    = ./logs

all: $(BUILD_DIR)/$(TARGET_EXEC)

MODULES = jigsaw node mcts

# Find all the C++ files we want to compile
SRCS = $(MODULES:%=$(BUILD_DIR)/%.cpp)

# Find all the .o files we want the compile to link
OBJS = $(MODULES:%=$(BUILD_DIR)/%.o)

# String substitution (suffix version without %).
DEPS = $(OBJS:.o=.d)

# The -MMD and -MP flags together generate Makefiles for us!
# These files will have .d instead of .o as the output.
CPPFLAGS = -Wall -Werror -O3 -I$(INC_DIR) -DNDEBUG -g -MMD -MP

# Count all files in LOGS_DIR that ends with .log
VALGRIND_LOG_SUFFIX = $(words $(wildcard $(LOGS_DIR)/*.log))

VALGRIND_FLAGS =    --leak-check=full \
                    --show-leak-kinds=all \
                    --track-origins=yes \
                    --log-file=$(LOGS_DIR)/$(VALGRIND_LOGS)_$(VALGRIND_LOG_SUFFIX).log $(BUILD_DIR)/$(TARGET_EXEC)

$(BUILD_DIR)/$(MAIN).o:$(MAIN).cpp
	@mkdir -p $(BUILD_DIR)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $(LDFLAGS) -c $< -o $@

# $@ evaluates to the target name
# $< evaluates to the first prerequisite 
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.cpp $(INC_DIR)/%.h
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -c $< -o $@

# $^ evaluates to all prerequisites
$(BUILD_DIR)/$(TARGET_EXEC): $(BUILD_DIR)/$(MAIN).o $(OBJS)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $^ -o $@

clean:
	@rm -rf $(BUILD_DIR)

valgrind: $(BUILD_DIR)/$(TARGET_EXEC)
	@mkdir -p $(LOGS_DIR)
	@valgrind $(VALGRIND_FLAGS)

run:
	$(BUILD_DIR)/$(TARGET_EXEC)

# Include the .d makefiles. The - at the front suppresses the errors of missing
# Makefiles. Initially, all the .d files will be missing, and we don't want those
# errors to show up.
-include $(DEPS)
