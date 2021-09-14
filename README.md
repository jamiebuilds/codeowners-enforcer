# codeowners-enforcer

> Enforce [`CODEOWNERS`](https://help.github.com/en/articles/about-code-owners) files on your repo

- Ensure that every file in your repo is owned by someone.
- Written in Rust for superb performance in even the largest repos.

## Install

**With npm:**

```sh
npm install --global codeowners-enforcer
```

**With Cargo:**

```sh
cargo install codeowners-enforcer
```

## Usage

`codeowners-enforcer` works by finding your `CODEOWNERS` file in a known
location as defined by [GitHub](https://help.github.com/en/articles/about-code-owners).

Then it walks through your file tree asserting that every file has a code owner.

If any files don't have a code owner, it will return their relative paths and
exit with `1`:

```sh
codeowners-enforcer
```

```txt
Oops! Found files without CODEOWNERS!

file.one
path/to/file.two
path/to/file.three

Fix: Please delete these files, move them, or add owners to them in /path/to/CODEOWNERS
```

If you want to return just the file paths, pass `--quiet` or `-q`:

```sh
codeowners-enforcer --quiet
```

```txt
one.txt
path/to/two.sh
path/to/three.py
```

If you want to ignore files, pass `--ignore <pattern>` or `-i`:

```sh
codeowners-enforcer --ignore "path/**/*.py"
```

```txt
one.txt
path/to/two.sh
```

You can also pass multiple ignores:

```sh
codeowners-enforcer -i "path/**/*.py" -i "path/**/*.sh"
```

```txt
one.txt
```

Alternatively, you can create a `.codeownersignore` file in your repository.
The file uses the same [syntax as a `.gitignore` file](https://git-scm.com/docs/gitignore#_pattern_format).

If you want to only check certain files, pass `<patterns...>`:

```sh
codeowners-enforcer "**/*.sh" "**/*.py"
```

```txt
path/to/two.sh
path/to/three.py
```
