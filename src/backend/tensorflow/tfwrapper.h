#pragma once
#include <cstddef>
#include <memory>

#include "rust/cxx.h"
#include "tensorflow/core/public/session.h"

class Session {
 public:
  Session(tensorflow::SessionOptions options,
          std::unique_ptr<tensorflow::Session> session);
  ~Session();
  size_t Hello() const;

 private:
  tensorflow::SessionOptions options_;
  std::unique_ptr<tensorflow::Session> session_;
  tensorflow::Allocator* allocator_ = nullptr;
};

std::unique_ptr<Session> CreateSession(rust::Str model_pb) noexcept(false);
