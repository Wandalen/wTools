# Contributing to `variadic_from`

We welcome contributions to the `variadic_from` project! By contributing, you help improve this crate for everyone.

## How to Contribute

1.  **Fork the Repository:** Start by forking the `wTools` repository on GitHub.
2.  **Clone Your Fork:** Clone your forked repository to your local machine.
    ```sh
    git clone https://github.com/your-username/wTools.git
    cd wTools/module/core/variadic_from
    ```
3.  **Create a New Branch:** Create a new branch for your feature or bug fix.
    ```sh
    git checkout -b feature/your-feature-name
    ```
    or
    ```sh
    git checkout -b bugfix/your-bug-fix
    ```
4.  **Make Your Changes:** Implement your changes, ensuring they adhere to the project's [code style guidelines](https://github.com/Wandalen/wTools/blob/master/doc/modules/code_style.md) and [design principles](https://github.com/Wandalen/wTools/blob/master/doc/modules/design_principles.md).
5.  **Run Tests:** Before submitting, ensure all existing tests pass and add new tests for your changes if applicable.
    ```sh
    cargo test --workspace
    ```
6.  **Run Clippy:** Check for linter warnings.
    ```sh
    cargo clippy --workspace -- -D warnings
    ```
7.  **Commit Your Changes:** Write clear and concise commit messages.
    ```sh
    git commit -m "feat(variadic_from): Add your feature description"
    ```
    or
    ```sh
    git commit -m "fix(variadic_from): Fix your bug description"
    ```
8.  **Push to Your Fork:**
    ```sh
    git push origin feature/your-feature-name
    ```
9.  **Open a Pull Request:** Go to the original `wTools` repository on GitHub and open a pull request from your branch. Provide a clear description of your changes and reference any related issues.

## Code Style and Design

Please adhere to the project's established [code style guidelines](https://github.com/Wandalen/wTools/blob/master/doc/modules/code_style.md) and [design principles](https://github.com/Wandalen/wTools/blob/master/doc/modules/design_principles.md). This ensures consistency and maintainability across the codebase.

## Reporting Issues

If you find a bug or have a feature request, please open an issue on our [GitHub Issues page](https://github.com/Wandalen/wTools/issues).

## Questions?

If you have any questions or need further assistance, feel free to ask on our [Discord server](https://discord.gg/m3YfbXpUUY).