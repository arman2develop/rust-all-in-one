## Rust-All-In-One
### Included All Toturials(Important Sections)

[![](https://tokei.rs/b1/arman2develop/rust-all-in-one?category=files&label=count%20of%20files)](https://github.com/arman2develop/rust-all-in-one).
[![codecov](https://codecov.io/gh/codecov/uploader/branch/master/graph/badge.svg?token=7ed4c74c-b784-464a-a9fe-fbea6dcb4cb8)](https://codecov.io/gh/arman2develop/rust-all-in-one)
[![Total alerts](https://img.shields.io/lgtm/alerts/g/codecov/uploader.svg?logo=lgtm&logoWidth=18)](https://lgtm.com/projects/g/arman2develop/rust-all-in-one/alerts/)

## How to Contribute
Contributions are always welcome! Please use the following guidelines when contributing to `rust-all-in-one`.

1. Fork `rust-all-in-one`
2. Clone your fork (`git clone https://github.com/$YOUR_USERNAME/rust-all-in-one && cd rust-all-in-one`)
3. Create a new branch (`git checkout -b new-branch`)
4. Make your changes, and commit (`git commit -am "your message"`)
 * I use a [conventional](https://github.com/ajoslin/conventional-changelog/blob/a5505865ff3dd710cf757f50530e73ef0ca641da/conventions/angular.md) changelog format so I can update my changelog using [clog](https://github.com/thoughtram/clog)
 * In addition to the conventions defined above, I also use `imp`, `wip`, `gr`.
 * Format your commit subject line using the following format: `TYPE(COMPONENT): MESSAGE` where `TYPE` is one of the following:
    - `feat` - A new feature
    - `imp` - An improvement to an existing feature
    - `perf` - A performance improvement
    - `tests` - Changes to the testing framework or tests only
    - `fix` - A bug fix
    - `refactor` - Code functionality doesn't change, but the underlying structure may
    - `style` - Stylistic changes only, no functionality changes
    - `wip` - A work-in-progress commit (Should typically be `git rebase`'ed away)
    - `chore` - Catch all or things that have to do with the build system, etc
    - `gr` - Graphics changes
 * The `COMPONENT` is optional, and may be a single file, directory, or logical component. Can be omitted if commit applies globally
5. Run the tests (`cargo test`)
6. `git rebase` into concise commits and remove `--fixup`s (`git rebase -i HEAD~NUM` where `NUM` is the number of commits back)
7. Push your changes back to your fork (`git push origin $your-branch`)
8. Create a pull request! (You can also create the pull request first, and we'll merge when ready. This a good way to discuss proposed changes.)
