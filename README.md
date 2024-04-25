# Git Automation Script

This script automates the process of updating, committing, and pushing changes to a Git repository. It performs the following tasks:

1. Adds all files recursively to the Git repository.
2. Retrieves the name of the current executable file.
3. Commits all changes to the Git repository with the executable file name as the commit message.
4. Pushes the committed changes to the remote Git repository.

## Prerequisites

- Rust programming language and Cargo (Rust's package manager) installed on your system.

## Usage

1. Clone or download the repository to your local machine.
2. Navigate to the directory containing the script.
3. Open a terminal or command prompt.
4. Run the following command to compile and execute the script:

    ```
    cargo run --release
    ```

    This command will compile the script in release mode and execute it.

## Troubleshooting

If you encounter any issues while running the script, please ensure that:

- You have Git installed on your system and properly configured.
- You have write permissions to the Git repository.
- There are changes in the repository to commit.
