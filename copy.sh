#! /bin/bash

cp src/main.rs src/main_$1.rs
ls src/

printf "%s\n" "fn main() {
    println!(\"\\n\");

    println!(\"\\n ラミン、あなたはとてもすばらし (0.0.1)\\n\");
}" > src/main.rs
