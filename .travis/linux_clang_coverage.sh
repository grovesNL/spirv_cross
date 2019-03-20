#!/bin/bash
export RUSTFLAGS="-C link-dead-code"
export CXX=clang++

cargo test --verbose --no-run --all-features
wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz
tar xzf master.tar.gz
cd kcov-master
mkdir build
cd build
cmake ..
make
make install DESTDIR=../../kcov-build
cd ../..
rm -rf kcov-master
for file in target/debug/*[^\.d]; do
  mkdir -p "target/cov/$(basename $file)"; ./kcov-build/usr/local/bin/kcov --include-path=spirv_cross/src --exclude-pattern=/.cargo,/usr/lib,.cpp,.hpp,vendor,tests,bindings.rs --verify "target/cov/$(basename $file)" "$file"
done
bash <(curl -s https://codecov.io/bash)
echo "Uploaded code coverage"
