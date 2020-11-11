#include "tfwrapper.h"

#include <cuda_runtime.h>

#include <stdexcept>

#include "symphony/src/backend/tensorflow/tfwrapper.rs.h"
#include "tensorflow/core/common_runtime/gpu/gpu_process_state.h"

namespace {
void unwrap(tensorflow::Status status) {
  if (!status.ok()) {
    throw std::runtime_error(status.error_message());
  }
}
}  // namespace

Tensor::Tensor(tensorflow::Tensor tensor) : tensor_(tensor) {}

std::unique_ptr<Tensor> Tensor::At(size_t index) const {
  CHECK_LT(index, static_cast<size_t>(tensor_.dim_size(0)));
  return std::make_unique<Tensor>(tensor_.Slice(index, index + 1));
}

void Tensor::CopyFrom(const rust::Vec<float>& src) {
  CHECK_EQ(static_cast<size_t>(tensorflow::DataTypeSize(tensor_.dtype())),
           sizeof(src[0]));
  CHECK_EQ(static_cast<size_t>(tensor_.NumElements()), src.size());
  void* pdst = tensor_.data();
  const void* psrc = src.data();
  size_t nbytes = sizeof(src[0]) * src.size();
  auto err = cudaMemcpy(pdst, psrc, nbytes, cudaMemcpyHostToDevice);
  CHECK(err == cudaSuccess) << cudaGetErrorString(err);
}

std::unique_ptr<std::vector<float>> Tensor::Read() const {
  auto dst = std::make_unique<std::vector<float>>();
  CHECK_EQ(static_cast<size_t>(tensorflow::DataTypeSize(tensor_.dtype())),
           sizeof(dst->at(0)));
  dst->resize(tensor_.NumElements());
  void* pdst = dst->data();
  const void* psrc = tensor_.data();
  size_t nbytes = sizeof(dst->at(0)) * dst->size();
  auto err = cudaMemcpy(pdst, psrc, nbytes, cudaMemcpyDeviceToHost);
  CHECK(err == cudaSuccess) << cudaGetErrorString(err);
  return dst;
}

std::unique_ptr<Session> CreateSession(SessionConfig config) {
  // Set session options
  tensorflow::SessionOptions options;
  auto* gpu_options = options.config.mutable_gpu_options();
  gpu_options->set_allocator_type("BFC");
  gpu_options->set_visible_device_list("0");

  // Create session
  tensorflow::Session* session = nullptr;
  unwrap(tensorflow::NewSession(options, &session));
  auto input_name = std::string(config.input_name);
  auto output_name = std::string(config.output_name);
  auto sess = std::make_unique<Session>(options, session, config.max_batch,
                                        input_name, output_name);

  // Load the model
  tensorflow::GraphDef graph_def;
  unwrap(tensorflow::ReadBinaryProto(options.env, std::string(config.model_pb),
                                     &graph_def));
  unwrap(session->Create(graph_def));

  // Allocate input tensor
  auto input_batch_tensor =
      sess->AllocateInputTensor(config.max_batch, config.input_shape);

  // Dry run to initialize the model
  std::vector<tensorflow::Tensor> output_tensors;
  auto input_tensor = input_batch_tensor.Slice(0, 1);
  unwrap(session->Run({{input_name, input_tensor}}, {output_name}, {},
                      &output_tensors));
  return sess;
}

Session::Session(tensorflow::SessionOptions options,
                 tensorflow::Session* session, size_t max_batch,
                 std::string input_name, std::string output_name)
    : options_(std::move(options)),
      session_(std::unique_ptr<tensorflow::Session>(session)),
      max_batch_(max_batch),
      input_name_(std::move(input_name)),
      output_name_(std::move(output_name)) {
  LOG(INFO) << "Constructing tfwrapper::Session";

  auto* process_state = tensorflow::GPUProcessState::singleton();
  allocator_ = process_state->GetGPUAllocator(options.config.gpu_options(),
                                              tensorflow::TfGpuId(0), 0);
}

Session::~Session() {
  session_->Close();
  LOG(INFO) << "Destructed tfwrapper::Session";
}

tensorflow::Tensor Session::AllocateInputTensor(
    size_t max_batch, const rust::Vec<size_t>& shape) {
  tensorflow::TensorShape tensor_shape;
  tensor_shape.AddDim(max_batch);
  for (auto n : shape) {
    tensor_shape.AddDim(n);
  }
  tensorflow::Tensor tensor(allocator_, tensorflow::DT_FLOAT, tensor_shape);
  allocated_tensors_.push_back(tensor);
  return tensor;
}

size_t Session::Hello() const { return 2333; }

std::unique_ptr<Tensor> Session::InputTensor() const {
  auto input_tensor = allocated_tensors_.at(0);
  return std::make_unique<Tensor>(input_tensor);
}

std::unique_ptr<Tensor> Session::Forward(size_t batch_size) {
  CHECK_LE(batch_size, max_batch_) << "More inputs than max_batch.";
  auto input_tensor = allocated_tensors_.at(0).Slice(0, batch_size);
  std::vector<tensorflow::Tensor> output_tensors;
  unwrap(session_->Run({{input_name_, input_tensor}}, {output_name_}, {},
                       &output_tensors));

  CHECK_EQ(output_tensors.size(), 1);
  auto output_tensor = output_tensors[0];
  CHECK_EQ(static_cast<size_t>(output_tensor.dim_size(0)), batch_size);
  return std::make_unique<Tensor>(output_tensor);
}
