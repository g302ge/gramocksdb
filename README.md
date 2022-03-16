# gramocksdb

rocksdb running in gramine libos experiment, create plain rocksdb operations for RW in practice

## Sealing 

The motivation is that when the enclave processes exits, the data within the boundary of the enclave will lost. So we have to a mechanism for reusing the data from disk.

Intuitively, the direct approach to protect the secret is using the key to encrypt the secrets and then decrypt from the encrypted. But there a question where the key from and how to keep it safe. Because of this question, Intel supply a machanism that derive key from measurement. There are two measurements.

1. MRENCLAVE 
2. MRSIGNER 

The first one uses the enclave code and secure log to derive the key every time the enclave launched, except the enclave update and its binary changed. But in this way the newer software could not access the old data on disk.


## Manifest syntax notes 

Gramine use the PAL to load a loader to load the user application. So the first binary will be loaded is the gramine libos binary and then will load the user application.

so the manifest will be set use the host path is sound.

```
loader.log_leverl = debug 
loader.log_file = "[HOST_PATH]" # for example ~/gramine.log
loader.entrypoint = "{{ gramine.runtimedir() }}" # point to current libsysdb.so path 
```

And then we could set the libos manifest maybe only the entrypoint

```
libod.entrypoint = "gramocksdb" 
# NOTE: this binary path should in the mounted file system in libos 
```

for example that 

```
fs.mounts = [
    {path = "/usr/bin/gramocksdb", uri = "file:gramocksdb"}
]
```


MRENCLAVE declares 

```
sgx.protected_mrenclave_files = [
    "launch_path/to/destation",
]
```
