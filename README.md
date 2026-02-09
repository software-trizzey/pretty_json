# Pretty JSON

A simple CLI tool that accepts a JSON filepath and formats its contents.

Importantly this tool doesn't validate JSON files and will fail when parsing invalid input.

Features:
- Override file contents by default
- Supports shortcut `--ouput|-o` that outputs changes to new file
- Select pretty or compact output via shortcut `--format|-f`; defaults to `pretty`
