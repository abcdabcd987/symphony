#pragma once
#include <cstddef>
#include <memory>
#include <vector>

#include "rust/cxx.h"
#include "tensorflow/core/framework/tensor.h"
#include "tensorflow/core/public/session.h"

class SessionConfig;

class Tensor {
 public:
  explicit Tensor(tensorflow::Tensor tensor);
  std::unique_ptr<Tensor> At(size_t index) const;
  void CopyFrom(const rust::Vec<float>& src);
  std::unique_ptr<std::vector<float>> Read() const;

 private:
  // This is a ref-counted pointer type.
  tensorflow::Tensor tensor_;
};

class Session {
 public:
  Session(tensorflow::SessionOptions options, tensorflow::Session* session,
          size_t max_batch, std::string input_name, std::string output_name);
  ~Session();
  std::unique_ptr<Tensor> InputTensor() const;
  std::unique_ptr<Tensor> Forward(size_t batch_size);
  size_t Hello() const;

 private:
  tensorflow::SessionOptions options_;
  std::unique_ptr<tensorflow::Session> session_;
  tensorflow::Allocator* allocator_ = nullptr;
  std::vector<tensorflow::Tensor> allocated_tensors_;
  const size_t max_batch_;
  const std::string input_name_;
  const std::string output_name_;

  friend std::unique_ptr<Session> CreateSession(SessionConfig config);
  tensorflow::Tensor AllocateInputTensor(size_t max_batch,
                                         const rust::Vec<size_t>& shape);
};

std::unique_ptr<Session> CreateSession(SessionConfig config);
