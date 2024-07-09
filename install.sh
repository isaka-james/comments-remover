#!/bin/bash

# Check if user is root
if [ "$(id -u)" -ne 0 ]; then
    echo "This script requires root privileges to install the tools."
    echo "Please run it with sudo:"
    echo "  sudo ./install.sh"
    exit 1
fi

# Check if uninstall flag is provided
if [ "$1" == "-uninstall" ]; then
    uninstall_tools
    exit 0
fi

# Compile Rust script
rustc remove-comment.rs -o remove-comment
chmod +x remove-comment

# Move compiled Rust script to system bin directory (assuming /usr/local/bin/)
sudo mv remove-comment /usr/local/bin/remove-comment

# Make sure remove-comment is executable from anywhere
sudo chmod +x /usr/local/bin/remove-comment

# Move bash script to system bin directory with executable permissions
chmod +x remove-comments
sudo mv remove-comments /usr/local/bin/remove-comments

# Provide user feedback
echo "remove-comment and remove-comments installed successfully!"
echo "To use:"
echo "  - Type 'remove-comment filename.extension' to process a single file."
echo "  - Type 'remove-comments' to process all relevant files in the current directory."
echo "  - Type 'remove-comments -uninstall' to uninstall the tool."
