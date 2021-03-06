#!/bin/bash

{
    echo "[[bin]]"
    echo "name = \"$1\""
    echo "path = \"src/$1.rs\""
} >> Cargo.toml

{
    echo "#![allow(unused_macros)]"
    echo ""
    echo "fn main() {"
    echo "    "
    echo "}"
} >> "src/$1.rs"