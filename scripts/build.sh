cargo build --release

mkdir -p ./releases
rm ./releases/*

for dir in ./target/*linux-gnu/
do
    echo Compressing $(basename $dir)
    zip ./releases/$(basename $dir).zip $dir/release/jpq -j > /dev/null
done