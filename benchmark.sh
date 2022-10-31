# generate random.bin and zero.bin if not created
SIZE=32M

echo "Generating random.bin and zero.bin of size $SIZE..."
if [ ! -f random.bin ]; then
    dd if=/dev/urandom of=random.bin bs=$SIZE count=1
fi

if [ ! -f zero.bin ]; then
    dd if=/dev/zero of=zero.bin bs=$SIZE count=1
fi
echo "Done!"

#cargo clean
cargo build --release ||  exit 1

# run the benchmark
echo "Benchmarking random.bin with entropy compressor"
time tar -c -I target/release/cn-entropy-compress -f test.tar.gz random.bin 
file test.tar.gz
echo "=============================================="
echo ""

echo "Benchmarking zero.bin with entropy compressor"
time tar -c -I target/release/cn-entropy-compress -f test.tar.gz zero.bin
file test.tar.gz
echo "=============================================="
echo ""

echo "Benchmarking random.bin with gzip"
time tar -czf test.tar.gz random.bin
file test.tar.gz
echo "=============================================="
echo ""

echo "Benchmarking zero.bin with gzip"
time tar -c -z -f test.tar.gz zero.bin
file test.tar.gz
echo "=============================================="

rm test.tar.gz
rm random.bin
rm zero.bin

