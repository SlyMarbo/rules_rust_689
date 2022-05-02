# Sample

This is an attempt to reproduce bazelbuild/rules_rust#689. See also bazelbuild/rules_rust#485 for the related issue of not (easily) being able to use generated source files as inputs to a `rust_library`.

Build with `bazel build //:lib`, which should succeed.

Build docs with `bazel build //:docs`, which will fail.
