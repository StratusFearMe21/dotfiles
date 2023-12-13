#include "rust/cxx.h"
#include <giac/config.h>
#include <giac/giac.h>
#include <memory>
#include <string>

namespace wrapper {
rust::String eval(const char *in, const giac::context *ctx);
std::unique_ptr<giac::context> new_ctx();
} // namespace wrapper
