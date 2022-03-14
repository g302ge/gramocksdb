SGX_SIGNER_KEY ?= ./sgx_private.pem
SGX ?= 1
USE_MUSL ?= 0 
BIN_NAME = gramocksdb

ifeq ($(USE_MUSL), 1) 
    BIN_FILE = target/x86_64-unknown-linux-musl/release/${BIN_NAME}
    CARGO_ARGS = --target x86_64-unknown-linux-musl
else
    BIN_FILE = target/release/${BIN_NAME}
    CARGO_ARGS =
endif

.PHONY: ${BIN_FILE}
${BIN_FILE}:
	cargo build --release ${CARGO_ARGS}

${BIN_NAME}: ${BIN_FILE}
	cp ${BIN_FILE} ${BIN_NAME}



.PHONY: sgx_gen_private_key
sgx_gen_private_key:
	openssl genrsa -out sgx_private.pem -3 3072

