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

```bash
  git remote add template https://github.com/adam-gaia/rust-template
  git fetch template
  git merge --allow-unrelated-histories template/main
```

Note that you will want to exclude at least

- TODO.md

## Cloning this repo

When cloning this repo, set up to pull CI files from https://github.com/epage/\_rust

```bash
  git remote add ci https://github.com/epage/_rust.git
  git fetch ci
  git merge --allow-unrelated-histories template/main
```

Grab the files in `.github/workflows/`

## Notes

The merging unrelated history process was inspired by [jonhoo's rust ci repo](https://github.com/jonhoo/rust-ci-conf/blob/main/.github/DOCS.md).
