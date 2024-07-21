# Assignment 1 - Behaviour-driven Development

With this assignment, you are expected to learn more about the Rust programming language and Behaviour-driven Development (BDD).

## Deadline

The deadline for this assignment is the 29th of July. Because there will be more assignments than just the first one, which will be due on the 12th of August, it is advisable to start early with this one.

## Submission

Send the `src` and `tests` folder as a zip archive to the tutor. **DO NOT INCLUDE THE `target` FOLDER.** One way of assuring that the assignment folder is as small as possible is to run `cargo clean` inside of it.

## Development Environment

- **Install Rust**: [https://www.rust-lang.org/](https://www.rust-lang.org/)
  - For Windows: MSVC may be required for Rust to work properly. Included in Visual Studio: [https://visualstudio.microsoft.com/downloads/](https://visualstudio.microsoft.com/downloads/)
- **Install rust-analyzer plugin**: [https://code.visualstudio.com/docs/languages/rust](https://code.visualstudio.com/docs/languages/rust)

## References

- **Rust Book**: [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)
  - **Install**: [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)
  - **Rustlings Rust Exercises/Tutorial**: [https://github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings)
  - **External Libraries (Crates)**: [https://doc.rust-lang.org/cargo/guide/dependencies.html](https://doc.rust-lang.org/cargo/guide/dependencies.html)
- **Cucumber**: [https://cucumber.io/docs/guides/overview/](https://cucumber.io/docs/guides/overview/)
  - **Gherkin**: [https://cucumber.io/docs/gherkin/reference/](https://cucumber.io/docs/gherkin/reference/)
  - **Cucumber.rs**: [https://github.com/cucumber-rs/cucumber](https://github.com/cucumber-rs/cucumber)
    - **Book**: [https://cucumber-rs.github.io/cucumber/current/](https://cucumber-rs.github.io/cucumber/current/)
  - **Regex**: [https://www.regular-expressions.info/quickstart.html](https://www.regular-expressions.info/quickstart.html)

## How to Run

To run all tests, execute:
```sh
cargo test
