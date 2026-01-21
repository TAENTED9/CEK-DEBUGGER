#!/bin/bash
# Cross-platform UPLC file creator for Unix-like systems (macOS, Linux, WSL)
# Usage: ./create_uplc.sh filename "uplc_content"

if [ $# -lt 2 ]; then
    echo "Usage: $0 <filename> <uplc_content>"
    echo ""
    echo "Examples:"
    echo "  $0 validator.uplc '(program 1.0.0 (lam x x))'"
    echo "  $0 test.uplc '(program 1.0.0 (con integer 42))'"
    exit 1
fi

FILENAME="$1"
CONTENT="$2"

# Create file with proper UTF-8 encoding (no BOM)
printf "%s" "$CONTENT" > "$FILENAME"

if [ $? -eq 0 ]; then
    echo "Created UPLC file: $FILENAME"
    echo "Content length: ${#CONTENT} characters"
    
    # Debug: show first 50 chars
    echo "Preview: $(echo "$CONTENT" | cut -c1-50)..."
else
    echo "Failed to create file"
    exit 1
fi
