#!/bin/sh

bindgen --c-naming \
  --enable-function-attribute-detection \
  --must-use-type CassError \
  --must-use-type CassErrorResult \
  --must-use-type CassFuture \
  --output src/ffi/cassandra.rs \
  src/ffi/wrapper.h
