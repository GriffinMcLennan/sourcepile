# Sourcepile

Simple cli application that allows you to quickly extract all contents of a folder or path into a single file with file name annotations for easy pasting into LLMs

Sourcepile walks your directory tree and gathers source files then writes them into one text file. Each section is annotated with its path relative to your project root so you know where each snippet came from

## installation

Install the tool globally in your system by running

```bash
cargo install sourcepile
```

After that you can run the tool from any folder using

```bash
sourcepile
```

You can also build and run locally with

```bash
cargo build
cargo run
```

## usage

```
sourcepile [path] [output] [--all] [--ignore-gitignore] [--max-lines N]
```

Arguments and options:

- **path**: Folder to scan. Defaults to the current directory.
- **output**: Name or path of the final dump file. Defaults to `sourcepile.txt` in the current folder.
- **--all**: Include every file regardless of extension.
- **--ignore-gitignore**: Bypass gitignore rules.
- **--max-lines N**: Limit reading to at most N lines per file. Zero or omission gives no limit.

## example

Gather code from a folder named src into demo.txt with a limit of five lines per file

```bash
sourcepile src demo.txt --all --ignore-gitignore --max-lines 5
```
