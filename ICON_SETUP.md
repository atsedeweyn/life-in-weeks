# Icon Setup Guide

Your new icon image has been saved to `src-tauri/icons/source-icon.png`.

## Option 1: Using Python Script (Recommended)

1. Install Pillow:
   ```bash
   pip3 install --user Pillow
   # OR if you have pip:
   pip install --user Pillow
   ```

2. Run the script:
   ```bash
   python3 generate-icons.py
   ```

3. For macOS ICNS file:
   - Go to: https://cloudconvert.com/png-to-icns
   - Upload: `src-tauri/icons/128x128@2x.png` (the 256x256 version)
   - Download and save as: `src-tauri/icons/icon.icns`

## Option 2: Using Online Tools (Easiest)

### PNG Files:
1. Go to: https://www.iloveimg.com/resize-image
2. Upload `source-icon.png`
3. Resize to:
   - 32x32 → save as `32x32.png`
   - 128x128 → save as `128x128.png`
   - 256x256 → save as `128x128@2x.png`

### ICO File (Windows):
1. Go to: https://convertio.co/png-ico/
2. Upload `source-icon.png` or `128x128.png`
3. Download and save as `icon.ico`

### ICNS File (macOS):
1. Go to: https://cloudconvert.com/png-to-icns
2. Upload `128x128@2x.png` (256x256 version)
3. Download and save as `icon.icns`

## Option 3: Manual (If you have image editing software)

1. Open your source image in an image editor (GIMP, Photoshop, etc.)
2. Export/resize to:
   - `32x32.png` - 32×32 pixels
   - `128x128.png` - 128×128 pixels
   - `128x128@2x.png` - 256×256 pixels
3. For ICO: Use an ICO converter or export as ICO with multiple sizes (16, 32, 48, 256)
4. For ICNS: Use macOS tools or online converters

## After Generating Icons

1. Make sure all files are in `src-tauri/icons/`:
   - ✅ `32x32.png`
   - ✅ `128x128.png`
   - ✅ `128x128@2x.png`
   - ✅ `icon.ico`
   - ✅ `icon.icns`

2. Rebuild your app:
   ```bash
   npm run tauri build
   ```

3. The new icons will appear in your built app!

## Quick Check

After generating, verify all files exist:
```bash
ls -lh src-tauri/icons/*.png src-tauri/icons/*.ico src-tauri/icons/*.icns
```
