OPENSSL_DIR=/usr/local/musl/ OPENSSL_INCLUDE_DIR=/usr/local/musl/include/ DEP_OPENSSL_INCLUDE=/usr/local/musl/include/ OPENSSL_LIB_DIR=/usr/local/musl/lib/ OPENSSL_STATIC=1 PKG_CONFIG_ALLOW_CROSS=true PKG_CONFIG_ALL_STATIC=true cargo build --target=x86_64-unknown-linux-musl

