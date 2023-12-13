#include "wrapper.h"
#include <fstream>
using namespace std;
using namespace giac;

namespace wrapper {
std::unique_ptr<giac::context> new_ctx() {
  std::unique_ptr<giac::context> ctx = std::make_unique<giac::context>();
  ofstream *filestream = 0;
  filestream = new ofstream("/dev/null");
  logptr(filestream, ctx.get());
  return ctx;
}

rust::String eval(const char *in, const giac::context *ctx) {
  try {
    gen e(in, ctx);
    e = eval(e, 1, ctx);
    stringstream s;
    s << e;

    return s.str();
  } catch (std::runtime_error e) {
    return "undef";
  }
}
} // namespace wrapper
