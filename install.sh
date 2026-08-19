#!/bin/bash

#Const
COMMAND_NAME="anime-cli"
INSTALL_DIR="$HOME/.local/bin"
REPO="Fanty1107/anime-cli"
BIN_FILE_NAME="anime-cli"
DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/$BIN_FILE_NAME"

echo "Starting the installation of '$COMMAND_NAME'..."

mkdir -p "$INSTALL_DIR"

echo "Downloading the latest version of do GitHub..."
if curl -s -L -o "$INSTALL_DIR/$COMMAND_NAME" "$DOWNLOAD_URL"; then
    echo "Download complete!"
else
    echo "Error downloading the file. Check your internet or if the Release exists on GitHub."
    exit 1
fi

echo "Configuring permissions"
chmod +x "$INSTALL_DIR/$COMMAND_NAME"
echo "Configuring add path to your shell"
#add to bash and zsh func
add_to_config() {
    local CONFIG_FILE=$1
    if [ -f "$CONFIG_FILE" ]; then
        if ! grep -q 'export PATH="$HOME/.local/bin:$PATH"' "$CONFIG_FILE"; then
            echo -e '\n# Add Anime CLI\nexport PATH="$HOME/.local/bin:$PATH"' >> "$CONFIG_FILE"
            echo "PATH add in $CONFIG_FILE"
        else
            echo "ℹ️  PATH is already set in $CONFIG_FILE"
        fi
    fi
}
add_to_config "$HOME/.bashrc"
add_to_config "$HOME/.zshrc"

# Fish
if command -v fish &> /dev/null; then
    fish -c "fish_add_path $INSTALL_DIR"
    echo "PATH config for fish"
fi

echo ""
echo "Completed installation!"
echo ": $COMMAND_NAME"

echo ""
echo "Installation completed"
echo "You can use this command: $COMMAND_NAME"
echo ""
echo "IMPORTANT: If the remote does not work immediately, restart your terminal,"
echo "    or make this commands below according to your shell:"
echo "    Bash: source ~/.bashrc"
echo "    Zsh:  source ~/.zshrc"
