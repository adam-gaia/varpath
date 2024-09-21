<div class="oranda-hide">

# TODO: Project name goes here

</div>

Project description goes here.

## Usage

1. Create a new project

```bash
  cargo new "${NAME}"
  cd "${NAME}"
```

2. Merge this repo with `--allow-unrelated-histories`. This way, we can pull in updates later.

```
  git remote add template https://github.com/adam-gaia/rust-template
  git fetch template
  git checkout
  git merge --allow-unrelated-histories template/main
```

Note that you will want to exclude at least

- TODO.md

This process was inspired by [jonhoo's rust ci repo](https://github.com/jonhoo/rust-ci-conf/blob/main/.github/DOCS.md).
