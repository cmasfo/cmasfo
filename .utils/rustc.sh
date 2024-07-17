
mkdir -p .build/rs
rustc code/rs/$1.rs -o .build/rs/$1 &&
.build/rs/$1
