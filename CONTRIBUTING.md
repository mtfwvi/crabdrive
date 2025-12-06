# Commiting

- Commit messages must be written in a descriptive, imperative style:
    - GOOD: `feat(server): Add new API endpoints`
    - BAD:  `new api endpoint`
- Commit messages must be written according to the
  [Conventional Commit specification](https://www.conventionalcommits.org/en/v1.0.0/):
  `<type>(optional scope): <description>`

# Branches & Pull requests

- The commit message for squash-merging pull requests must contain the issue number(s) it references:
  `<type>(optional scope): <description> (<issues>)`
- Branches with developer name abbreviation as prefix and issue number as suffix, e.g. for John Smith:
  `smi/fix-unsafe-code-#23`
- Before merging pull requests, the code must be formatted according to the style guidelines (using cargo format)
- Minimum of TWO reviewers are required before a pull request can be merged into master