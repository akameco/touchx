# touchx

> Better touch. Efficient file creation with templates.

If you save files such as LICENSE or .editorconfig, you can create files with similar content to the saved file when using the same file name.
The saved templates are stored in the appropriate folder for each operating system based on the XDG Base Directory Specification.
For more information, please refer to https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html

## Install

```
$ cargo install touchx
```

## Usage

```sh
$ touchx sample.txt
$ ls
sample.txt
$ echo 'hello' >> sample.txt
$ cat sample.txt
hello
$ touchx --save sample.txt
$ rm sample.txt
$ touchx sample.txt
$ cat sample.txt
hello
```

```sh
$ touchx --help
Better touch. Efficient file creation with templates.

Usage: touchx [OPTIONS] <FILE>

Arguments:
  <FILE>  target file

Options:
      --save     save the target file
  -h, --help     Print help
  -V, --version  Print version

$ toux
```

## License

MIT