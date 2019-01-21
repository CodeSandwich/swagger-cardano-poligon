# swagger-cardano-poligon
Test flight of Swagger-generated Rust server

Building steps:

0. Download link borked
0. Half every 18446744073709551615 to 9223372036854775807 in API file

0. cargo install cargo-swagger
0. cargo swagger cardano-node-api.swagger.json <outdir>
0. sudo chown -R `whoami` <outdir>
0. Fix cargo.toml until it compiles and makes sense

If you're on Ubuntu 18.10:
0. sudo apt install libssl1.0-dev
0. OPENSSL_LIB_DIR='/usr/lib/x86_64-linux-gnu/openssl-1.0.0/engines' cargo build
