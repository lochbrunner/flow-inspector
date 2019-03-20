#include <hiredis/hiredis.h>
#include <iostream>
#include <sstream>
#include <string>

using std::cerr;
using std::string;

void update(redisContext *context, const string &value) {
  redisReply *reply = nullptr;
  redisAppendCommand(context, "LPUSH log %s", value.c_str());
  redisAppendCommand(context, "PUBLISH log %s", value.c_str());
  redisGetReply(context, reinterpret_cast<void **>(&reply));  // reply for SET

  switch (reply->type) {
    case REDIS_REPLY_STATUS:
      cerr << "Reply status: " << reply->str << std::endl;
      break;
    case REDIS_REPLY_ERROR:
      cerr << "Reply error: " << reply->str << std::endl;
      return;
    case REDIS_REPLY_INTEGER:
      cerr << "Integer: " << reply->integer << std::endl;
      return;
    default:
      break;
  }
  freeReplyObject(reply);
}

#define SER_STRING(stream, property) \
  stream << "\"" << #property << "\": \"" << property << "\""

#define SER_STRING_AP(stream, property) \
  stream << "\"" << #property << "\": \"" << property << "\",\n"

struct Status {
  string node_id;
  string status;
  string json() {
    std::stringstream st;
    st << "{\n";
    SER_STRING_AP(st, node_id);
    SER_STRING(st, status);
    st << "\n}";
    return st.str();
  }
};

struct Connection {
  string topic;
  string sending_node_id;
  string receiving_node_id;

  string json() {
    std::stringstream st;
    st << "{\n";
    SER_STRING_AP(st, topic);
    SER_STRING_AP(st, sending_node_id);
    SER_STRING(st, receiving_node_id);
    st << "\n}";
    return st.str();
  }
};

int main(int argn, char **argv) {
  redisContext *context = redisConnect("127.0.0.1", 6379);
  if (context == NULL || context->err) {
    if (context) {
      cerr << "Error: " << context->errstr << "\n";
    } else {
      cerr << "Can't allocate redis context\n";
    }
  }

  Status status{"node-d", "ok"};
  update(context, status.json());

  Connection connection{"topic-a", "node-a", "node-b"};
  update(context, connection.json());

  redisFree(context);
  return 0;
}