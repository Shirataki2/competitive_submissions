echo "[[bin]]" >> Cargo.toml
echo "name = \"$1\"" >> Cargo.toml
echo "path = \"src/$1.rs\"" >> Cargo.toml

touch "src/$1.rs"

echo "fn main() {" >> src/$1.rs
echo "    " >> src/$1.rs
echo "}" >> src/$1.rs