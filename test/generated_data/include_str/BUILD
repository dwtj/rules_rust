load("//rust:rust.bzl", "rust_test")

genrule(
    name = "generated_data",
    outs = ["generated_data.txt"],
    cmd_bash = 'echo "world" > "$(@)"',
)

rust_test(
    name = "include_str",
    srcs = ["include_str.rs"],
    data = [
        "source_data.txt",
        "generated_data",
    ],
)
