VERS=1.0.2o && \
    curl -O https://www.openssl.org/source/openssl-$VERS.tar.gz && \
    tar xvzf openssl-$VERS.tar.gz && cd openssl-$VERS && \
    env CC=musl-gcc ./config --prefix=/usr/local/musl && \
    env C_INCLUDE_PATH=/usr/local/musl/include/ make depend && \
    make && sudo make install && \
cd .. && rm -rf openssl-$VERS.tar.gz openssl-$VERS
