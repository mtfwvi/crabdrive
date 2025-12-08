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
- Branches prefixed with developer name abbreviation and issue number, e.g. for John Smith working on issue #23:
  `smi/23-fix-unsafe-code`
- Before merging pull requests, the code must be formatted according to the style guidelines (using cargo format)
- Minimum of TWO reviewers are required before a pull request can be merged into master