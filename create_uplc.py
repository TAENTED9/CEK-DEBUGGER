#!/usr/bin/env python3
"""
Cross-platform UPLC file creator (Windows, macOS, Linux)
Works with or without PowerShell, Bash, etc.

Usage:
    python3 create_uplc.py <filename> <uplc_content>

Examples:
    python3 create_uplc.py validator.uplc "(program 1.0.0 (lam x x))"
    python3 create_uplc.py test.uplc "(program 1.0.0 (con integer 42))"
"""

import sys
import os
from pathlib import Path


def create_uplc_file(filename, content):
    """
    Create UPLC file with proper UTF-8 encoding (no BOM)
    Works on all platforms: Windows, macOS, Linux
    
    Args:
        filename (str): Path and filename for the UPLC file
        content (str): UPLC program content
    
    Returns:
        bool: True if successful, False otherwise
    """
    try:
        # Convert to Path object for cross-platform compatibility
        filepath = Path(filename)
        
        # Write with explicit UTF-8 encoding (no BOM)
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        
        # Verify file was created
        if filepath.exists():
            file_size = filepath.stat().st_size
            print(f"✓ Created UPLC file: {filepath}")
            print(f"✓ File size: {file_size} bytes")
            print(f"✓ Content preview: {content[:60]}{'...' if len(content) > 60 else ''}")
            return True
        else:
            print(f"✗ File was not created")
            return False
            
    except Exception as e:
        print(f"✗ Error creating file: {e}")
        return False


def create_multiple_uplc_files(files):
    """
    Create multiple UPLC files at once
    
    Args:
        files (list): List of tuples [(filename, content), ...]
    
    Returns:
        bool: True if all successful
    """
    all_success = True
    for filename, content in files:
        if not create_uplc_file(filename, content):
            all_success = False
    
    if all_success:
        print(f"\n✓ Successfully created {len(files)} UPLC file(s)")
    return all_success


def main():
    """Main entry point"""
    if len(sys.argv) < 3:
        print(__doc__)
        sys.exit(1)
    
    filename = sys.argv[1]
    content = sys.argv[2]
    
    success = create_uplc_file(filename, content)
    sys.exit(0 if success else 1)


if __name__ == '__main__':
    main()
