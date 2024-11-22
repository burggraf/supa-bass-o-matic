#!/bin/bash

# Exit on error
set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting build process for Supa Bass-a-matic...${NC}"

# Step 1: Install dependencies
echo -e "${YELLOW}Installing dependencies...${NC}"
npm install
(cd src-tauri && cargo check)

# Step 2: Build the app
echo -e "${YELLOW}Building the application...${NC}"
npm run build
cargo tauri build

# Step 3: Sign the app
echo -e "${YELLOW}Preparing to sign the application...${NC}"

# Get the path to the app bundle
APP_PATH="src-tauri/target/release/bundle/macos/Supa Bass-a-matic.app"

# Check if we have a valid signing identity in the login keychain
if ! security find-identity -v -p codesigning login.keychain-db | grep -q "Developer ID Application"; then
    echo "No valid Developer ID Application certificate found in login keychain!"
    echo "Please ensure you have a valid Apple Developer ID certificate installed in your login keychain."
    exit 1
fi

# Get the first valid Developer ID Application certificate from login keychain
SIGNING_IDENTITY=$(security find-identity -v -p codesigning login.keychain-db | grep "Developer ID Application" | head -n 1 | cut -d '"' -f 2)

echo -e "${YELLOW}Signing with identity: $SIGNING_IDENTITY${NC}"

# Sign the app using the login keychain
echo -e "${YELLOW}Signing the application...${NC}"
codesign --force --deep --options runtime --keychain ~/Library/Keychains/login.keychain-db --sign "$SIGNING_IDENTITY" "$APP_PATH"

# Verify the signature
echo -e "${YELLOW}Verifying signature...${NC}"
codesign --verify --deep --strict "$APP_PATH"
spctl --assess --type execute --verbose "$APP_PATH"

# Create a DMG (optional)
echo -e "${YELLOW}Creating DMG...${NC}"
if command -v create-dmg &> /dev/null; then
    create-dmg \
        --volname "Supa Bass-a-matic" \
        --volicon "src-tauri/icons/icon.icns" \
        --window-pos 200 120 \
        --window-size 600 400 \
        --icon-size 100 \
        --icon "Supa Bass-a-matic.app" 175 190 \
        --hide-extension "Supa Bass-a-matic.app" \
        --app-drop-link 425 190 \
        "target/Supa Bass-a-matic.dmg" \
        "$APP_PATH"
else
    echo "create-dmg not found. Skipping DMG creation."
    echo "To install create-dmg: brew install create-dmg"
fi

echo -e "${GREEN}Build and signing process completed!${NC}"
echo -e "${GREEN}Your app is ready at: $APP_PATH${NC}"
if [ -f "target/Supa Bass-a-matic.dmg" ]; then
    echo -e "${GREEN}DMG file created at: target/Supa Bass-a-matic.dmg${NC}"
fi
