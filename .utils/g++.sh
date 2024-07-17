
cmd="g++"
type="cc"

build=".build/$type"
bin_target="$build/$1"

mkdir -p $build
$cmd code/$type/$1.$type -o $bin_target &&
$bin_target
