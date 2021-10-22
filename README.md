# Writing 

> a document-code sync tools for document engineering.

language parse support by [guarding](https://github.com/inherd/guarding) with [tree-sitter](https://github.com/tree-sitter/tree-sitter)

current supported language: Java, JavaScript, Rust

## Usage

### Install

install with Rust Cago 

```
cargo install writing
```

or download from: [release](https://github.com/inherd/writing/releases)

### Run

usage:

```
writing -p README.md
```

all command:

```bash
OPTIONS:
    -h, --help               Print help information
    -o, --output <OUTPUT>    [default: out.md]
    -p, --path <PATH>        [default: README.md]
    -V, --version            Print version information
```

## Samples

by Lines

```writing
// doc-code: file("src/lib.rs").line()[1, 5]
```

by Section

```writing
// doc-section: file("src/lib.rs").section("section1")
```

by Function

```writing
// doc-func: file("src/lib.rs").func()["pre_process_file", "process_file"]
```

## Documents

[API 库的文档体系支持：主流编程语言的文档设计](https://www.phodal.com/blog/api-ducumentation-design-dsl-base/)

[文档工程体验设计：重塑开发者体验](https://www.phodal.com/blog/documentation-enginnering-experience-design/)

License
---

@ 2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
