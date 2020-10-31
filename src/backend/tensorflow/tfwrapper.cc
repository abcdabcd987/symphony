#include "tfwrapper.h"

#include <stdexcept>

#include "tensorflow/core/common_runtime/gpu/gpu_process_state.h"

namespace {
void unwrap(tensorflow::Status status) noexcept(false) {
  if (!status.ok()) {
    throw std::runtime_error(status.error_message());
  }
}
}  // namespace

std::unique_ptr<Session> CreateSession(rust::Str model_pb) noexcept(false) {
  // Set session options
  tensorflow::SessionOptions options;
  auto* gpu_options = options.config.mutable_gpu_options();
  gpu_options->set_allocator_type("BFC");
  gpu_options->set_visible_device_list("0");

  // Create session
  tensorflow::Session* psession = nullptr;
  unwrap(tensorflow::NewSession(options, &psession));
  std::unique_ptr<tensorflow::Session> session(psession);

  // Load model
  tensorflow::GraphDef graph_def;
  unwrap(tensorflow::ReadBinaryProto(options.env, std::string(model_pb),
                                     &graph_def));
  unwrap(session->Create(graph_def));

  return std::make_unique<Session>(std::move(options), std::move(session));
}

Session::Session(tensorflow::SessionOptions options,
                 std::unique_ptr<tensorflow::Session> session)
    : options_(std::move(options)), session_(std::move(session)) {
  LOG(INFO) << "Constructor";

  auto* process_state = tensorflow::GPUProcessState::singleton();
  allocator_ = process_state->GetGPUAllocator(options.config.gpu_options(),
                                              tensorflow::TfGpuId(0), 0);
}

Session::~Session() {
  LOG(INFO) << "Destructor";
  session_->Close();
}

size_t Session::Hello() const { return 2333; }
