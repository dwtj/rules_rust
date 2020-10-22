load("//rust:rust.bzl", "rust_test")
load("//cargo:cargo_build_script.bzl", "cargo_build_script")

genrule(
    name = "generated_data",
    outs = ["generated_data.txt"],
    cmd_bash = 'echo "world" > "$(@)"',
)

cargo_build_script(
    name = "do_nothing_cargo_build_script",
    srcs = ["do_nothing_cargo_build_script.rs"],
)

# TODO(dwtj): This test demonstrates a workaround for issue #459. It relies on
#  the assumption that the Bazel-generated file `generated_data.txt` will be a
#  sibling of the Cargo build script's `OUT_DIR` during the `rustc` action. This
#  seems like it might be a problematic assumption. We ought to find a more
#  rigorous and Bazel-y solution.
rust_test(
    name = "include_str",
    srcs = [
        "include_str.rs",
    ],
    data = [
        "source_data.txt",
        "generated_data.txt",
    ],
    deps = [
        # NOTE(dwtj): We only include this dependency as part of a workaround
        #  for issue #459. Because we have included a `cargo_build_script`
        #  target as a dep, the `OUT_DIR` environment variable will be set when
        #  we compile this target. We can use this environment variable's value,
        #  to locate `generated_data.txt` for use in `include_str!()`.
        ":do_nothing_cargo_build_script"],
)
