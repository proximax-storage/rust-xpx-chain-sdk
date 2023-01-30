## Generate flatbuffers transactions

`flatc --rust schema_aggregate_transaction.fbs`

Docs: http://google.github.io/flatbuffers/

You can install `flatc` manually

```
git clone https://github.com/google/flatbuffers.git
cd flatbuffers
cmake -G "Unix Makefiles"
make
cd -
```

You can use syntax like:

```$xslt
./flatbuffers/flatc --defaults-json --gen-includes --gen-mutable --gen-all --rust-module-root-file --rust *.fbs
```


