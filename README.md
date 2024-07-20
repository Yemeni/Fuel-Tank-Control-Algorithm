Assignment 1 - Behaviour-driven development
With this assignment you are expected to learn more about the Rust programming
language and Behaviour-driven development (BDD).
Deadline
The deadline for this assignment will be the 29th of July.
Because there will be more assignments than just the first one, which will be
due on the 12th of august, it is advisable to start early with this one.
Submission
Send the src and tests folder as a zip archive at sven.friedrich@dlr.de.
DO NOT INCLUDE THE target FOLDER. One way of assuring that the
assignment folder is as small as possible, is to run cargo clean inside of it.
Development Environment
• Install Rust: https://www.rust-lang.org/
– For Windows: MSVC may be required for Rust to work properly
Included in Visual Studio: https://visualstudio.microsoft.com/
downloads/
• Install rust-analyzer plugin: https://code.visualstudio.com/docs/
languages/rust
References
• Rust Book: https://doc.rust-lang.org/book/
– Install: https://www.rust-lang.org/learn/get-started
– Rustlings Rust Exercises/Tutorial https://github.com/rustlang/rustlings
– External Libraries (Crates): https://doc.rust-lang.org/cargo/guide/
dependencies.html
• Cucumber: https://cucumber.io/docs/guides/overview/
– Gherkin: https://cucumber.io/docs/gherkin/reference/
– Cucumber.rs: https://github.com/cucumber-rs/cucumber
∗ Book: https://cucumber-rs.github.io/cucumber/current/
– Regex: https://www.regular-expressions.info/quickstart.html
How to run
To run all tests execute cargo test
Run your tests
To run only your tests execute cargo test --test control_algorithm
1
Goals
In this assignment you will develop a fuel tank control algorithm.
Procedure
1. IMPLEMENT TESTS FIRST
With BDD you are expected to write your tests first. This means that you
think about what the ultimate goal is that you want to achieve with your
control algorithm. In this case we want a control algorithm which prevents
the fuel tanks from overflowing.
1. Write your gherkin features into the tests/features/control_algorithm.feature
file. You can take some inspiration from the tests/features/simulator.feature
2. Execute your tests (cargo test --test control_algorithm) and
make sure all your gherkin steps parse. You should add functions
for parsing your gherkin steps to the tests/control_algorithm.rs
file (take a look at the tests/simulator.rs file for reference). It is
desirable that your tests will fail at this stage, because you did not
implement your control algorithm yet. Having failing tests at this
stage is the essence of BDD. (“fail” does not mean “not parsed”, make
sure that none of the gherkin steps are skipped)
2. Implement your control algorithm
A file and struct is already prepared for you to start with the implementation of the algorithm. In the src/control_algorithm.rs file you will find
the SimpleControlAlgorithm which will be your algorithm. The todo()
in this file should be replaced with your implementation.
1. Implement your algorithm
2. Run your tests (cargo test --test control_algorithm)
3. If any test fails, repeat.
2
