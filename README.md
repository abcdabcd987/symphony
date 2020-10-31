# symphony

[![Actions Status](https://github.com/abcdabcd987/symphony/workflows/CI/badge.svg)](https://github.com/abcdabcd987/symphony/actions)
[![codecov](https://codecov.io/gh/abcdabcd987/symphony/branch/master/graph/badge.svg)](https://codecov.io/gh/abcdabcd987/symphony/)


# Build

```bash
wget https://github.com/abcdabcd987/libtensorflow_cc/releases/download/v2.3.1/libtensorflow_cc-v2.3.1-gpu-cuda10.1-cudnn7-build20201029.tar.bz2
tar xf libtensorflow_cc-v2.3.1-gpu-cuda10.1-cudnn7-build20201029.tar.bz2
mv libtensorflow_cc-v2.3.1 libtensorflow_cc-v2.3.1-gpu

cargo build
cargo test
```
