arg1=$1
cargo run --bin minify <$arg1 >minified
cargo run --bin request <minified
