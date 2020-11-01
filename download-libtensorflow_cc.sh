#!/bin/sh
set -uxe

curl -L -o libtensorflow_cc-v2.3.1-gpu-cuda10.1-cudnn7-build20201029.tar.bz2 https://github.com/abcdabcd987/libtensorflow_cc/releases/download/v2.3.1/libtensorflow_cc-v2.3.1-gpu-cuda10.1-cudnn7-build20201029.tar.bz2
tar xf libtensorflow_cc-v2.3.1-gpu-cuda10.1-cudnn7-build20201029.tar.bz2
mv libtensorflow_cc-v2.3.1 libtensorflow_cc-v2.3.1-gpu
