#!bin/bash

cargo run -p prisma-cli --release $1

file="./prisma/src/lib.rs"

sed -i '4d' $file
sed -i '3s/.*/pub static DATAMODEL_STR: \&\'"'"'static str = include_str!("..\/schema.prisma");\n/' "$file"

cargo fix --lib -p db --allow-dirty
