# make

CC = gcc
CFLAGS = -g -Wall -pthread

CLIENT_INC = client/include
CLIENT_SRC = client/source

SERVER_INC = server/include
SERVER_SRC = server/source

COMMON_INC = common/include
COMMON_SRC = common/source

BUILD_DIR = build

CLIENT_PROG = $(BUILD_DIR)/client/client
SERVER_PROG = $(BUILD_DIR)/server/server

.PHONY: target create_folder clean

target: create_folder $(CLIENT_PROG) $(SERVER_PROG)
	@echo "build \"$(CLIENT_PROG)\" and \"$(SERVER_PROG)\" finish"

$(CLIENT_PROG): $(CLIENT_SRC)/client.c $(COMMON_SRC)/stack_int.c $(COMMON_SRC)/thread_pool.c $(COMMON_SRC)/fileList.c $(COMMON_SRC)/strUtil.c $(COMMON_SRC)/util.c
	$(CC) $(CFLAGS) $(addprefix -I, $(CLIENT_INC) $(COMMON_INC)) $^ -o $@

$(SERVER_PROG): $(SERVER_SRC)/server.c $(COMMON_SRC)/stack_int.c $(COMMON_SRC)/thread_pool.c $(COMMON_SRC)/strUtil.c $(COMMON_SRC)/util.c
	$(CC) $(CFLAGS) $(addprefix -I, $(SERVER_INC) $(COMMON_INC)) $^ -o $@ # -lmysqlclient


create_folder:
	-mkdir -p $(BUILD_DIR)
	-mkdir -p $(BUILD_DIR)/client
	-mkdir -p $(BUILD_DIR)/server

clean:
	-rm $(CLIENT_PROG)
	-rm $(SERVER_PROG)

