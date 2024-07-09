# Remove Comments Tool 🧹

This tool helps you clean up your code by removing comments from files in various programming languages. It supports languages like C++, Java, JavaScript, Rust, and more. Install it once and use it anywhere!

## Installation

To install the tool, make sure you have Rust installed:
1. Clone the repository:
   ```bash
   git clone https://github.com/isaka-james/comments-remover.git
   cd comments-remover
   ```

2. Run the installation script with sudo:
   ```bash
   sudo ./install.sh
   ```

3. That's it! Now you can use `remove-comment` for single files and `remove-comments` for entire directories.

## Usage

### Removing Comments from a Single File
To remove comments from a specific file, use `remove-comment` followed by the filename with its extension:
```bash
remove-comment filename.js
```

### Removing Comments from Multiple Files in a Directory
To clean up all files of supported types in the current directory and its subdirectories, simply run:
```bash
remove-comments
```

### Uninstalling

If you ever need to uninstall the tool, simply run:
```bash
remove-comments -uninstall
```

This will remove both `remove-comment` and `remove-comments` from your system.

## Contributing

Contributions are welcome! Fork the repository and submit a pull request with your improvements.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

<img src="https://komarev.com/ghpvc/?username=comments-remover&label=Comments-Remover&color=0e75b6&style=flat" alt="since 9th July,2024" />
