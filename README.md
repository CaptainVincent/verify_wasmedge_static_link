# Release note

These libraries are self-built using docker build, based on the fmt-patch branch of git@github.com:WasmEdge/WasmEdge.git `beb8bf0d21ef2bf535337af971b371e833e4929b`

# Test Steps

1. Clone this repo
2. export WASMEDGE_STANDALONE_ARCHIVE matches your platform
  ex. WASMEDGE_STANDALONE_ARCHIVE=$PWD/WasmEdge-0.14.1-1-fmt-patch-debian11_x86_64_static.tar.gz
3. cargo build
