# gramocksdb

rocksdb running in gramine libos experiment, create plain rocksdb operations for RW in practice


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
libod.entrypoint = "gramocksdb" # NOTE: this binary path should in the mounted file system in libos 
```

for example that 

```
fs.mounts = [
    {path = "/usr/bin/gramocksdb", uri = "file:gramocksdb"}
]
```



