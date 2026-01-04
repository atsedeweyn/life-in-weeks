#!/usr/bin/env python3
"""
Generate Tauri app icons from a source image.
Requires: pip install Pillow
"""

import sys
from pathlib import Path

try:
    from PIL import Image
except ImportError:
    print("Error: Pillow is required. Install it with: pip install Pillow")
    sys.exit(1)

ICONS_DIR = Path("src-tauri/icons")
SOURCE_IMAGE = ICONS_DIR / "source-icon.png"

def generate_png_icons():
    """Generate PNG icons at required sizes."""
    print("Loading source image...")
    img = Image.open(SOURCE_IMAGE)
    
    # Convert to RGBA if needed
    if img.mode != 'RGBA':
        img = img.convert('RGBA')
    
    sizes = [
        (32, 32, "32x32.png"),
        (128, 128, "128x128.png"),
        (256, 256, "128x128@2x.png"),
    ]
    
    print("Generating PNG icons...")
    for width, height, filename in sizes:
        resized = img.resize((width, height), Image.Resampling.LANCZOS)
        output_path = ICONS_DIR / filename
        resized.save(output_path, "PNG")
        print(f"  ✓ Created {filename} ({width}x{height})")

def generate_ico():
    """Generate Windows ICO file with multiple sizes."""
    print("Generating Windows ICO file...")
    img = Image.open(SOURCE_IMAGE)
    
    if img.mode != 'RGBA':
        img = img.convert('RGBA')
    
    # ICO format requires multiple sizes
    sizes = [16, 32, 48, 256]
    icons = []
    
    for size in sizes:
        resized = img.resize((size, size), Image.Resampling.LANCZOS)
        icons.append(resized)
    
    output_path = ICONS_DIR / "icon.ico"
    icons[0].save(
        output_path,
        format='ICO',
        sizes=[(s, s) for s in sizes]
    )
    print(f"  ✓ Created icon.ico with sizes: {sizes}")

def generate_icns_info():
    """Print instructions for generating ICNS file."""
    print("\n" + "="*60)
    print("ICNS file (macOS) generation:")
    print("="*60)
    print("ICNS files are macOS-specific and harder to generate on Linux.")
    print("\nOption 1: Use online converter")
    print("  - Go to: https://cloudconvert.com/png-to-icns")
    print("  - Upload: src-tauri/icons/128x128@2x.png (256x256)")
    print("  - Download and save as: src-tauri/icons/icon.icns")
    print("\nOption 2: Use macOS (if available)")
    print("  - Use 'iconutil' command on macOS")
    print("  - Or use online tools like: https://convertio.co/png-icns/")
    print("="*60)

def main():
    if not SOURCE_IMAGE.exists():
        print(f"Error: Source image not found: {SOURCE_IMAGE}")
        sys.exit(1)
    
    print("Generating Tauri app icons...")
    print(f"Source: {SOURCE_IMAGE}\n")
    
    generate_png_icons()
    print()
    generate_ico()
    print()
    generate_icns_info()
    
    print("\n✓ Icon generation complete!")
    print("\nNext steps:")
    print("1. Generate icon.icns using one of the methods above")
    print("2. Rebuild your Tauri app to see the new icons")

if __name__ == "__main__":
    main()
