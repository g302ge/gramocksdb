
SGX ?= 1

USE_MUSL ?= 0 

BIN_NAME = gramocksdb

.PHONY: sgx_gen_private_key
sgx_gen_private_key:
	openssl genrsa -out sgx_private.pem -3 3072

