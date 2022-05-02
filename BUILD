load("@rules_rust//rust:defs.bzl", "rust_doc", "rust_library")

rust_library(
    name = "lib",
    srcs = [
        "lib.rs",
        "source.rs",
    ],
    compile_data = [":gen_rust"],
    edition = "2021",
    rustc_env = {
        "GENERATED_RS": "$(location :gen_rust)",
    },
)

genrule(
    name = "gen_rust",
    srcs = [],
    outs = ["generated.rs"],
    cmd = "printf '%s\n' 'pub const GENERATED: () = ();' >$@",
)

rust_doc(
    name = "docs",
    crate = ":lib",
)