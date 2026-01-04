#!/usr/bin/env node
/**
 * Generate Tauri app icons from a source image.
 * Requires: npm install sharp
 */

const fs = require('fs');
const path = require('path');

const ICONS_DIR = path.join(__dirname, 'src-tauri', 'icons');
const SOURCE_IMAGE = path.join(ICONS_DIR, 'source-icon.png');

async function generateIcons() {
  let sharp;
  try {
    sharp = require('sharp');
  } catch (err) {
    console.error('Error: sharp is required. Install it with:');
    console.error('  npm install sharp');
    process.exit(1);
  }

  if (!fs.existsSync(SOURCE_IMAGE)) {
    console.error(`Error: Source image not found: ${SOURCE_IMAGE}`);
    process.exit(1);
  }

  console.log('Generating Tauri app icons...');
  console.log(`Source: ${SOURCE_IMAGE}\n`);

  // Generate PNG icons
  const pngSizes = [
    { size: 32, filename: '32x32.png' },
    { size: 128, filename: '128x128.png' },
    { size: 256, filename: '128x128@2x.png' },
  ];

  console.log('Generating PNG icons...');
  for (const { size, filename } of pngSizes) {
    const outputPath = path.join(ICONS_DIR, filename);
    await sharp(SOURCE_IMAGE)
      .resize(size, size, { fit: 'contain', background: { r: 255, g: 255, b: 255, alpha: 0 } })
      .png()
      .toFile(outputPath);
    console.log(`  ✓ Created ${filename} (${size}x${size})`);
  }

  // Generate ICO file (Windows) - create PNG for conversion
  console.log('\nPreparing Windows ICO file...');
  const icoPng = path.join(ICONS_DIR, 'icon-256-for-ico.png');
  await sharp(SOURCE_IMAGE)
    .resize(256, 256, { fit: 'contain', background: { r: 255, g: 255, b: 255, alpha: 0 } })
    .png()
    .toFile(icoPng);
  console.log(`  ✓ Created ${path.basename(icoPng)} for ICO conversion`);
  console.log('  → Convert to ICO: https://convertio.co/png-ico/');
  console.log('     Upload the PNG above and download as icon.ico');

  console.log('\n' + '='.repeat(60));
  console.log('ICNS file (macOS) generation:');
  console.log('='.repeat(60));
  console.log('ICNS files require macOS-specific tools.');
  console.log('\nOption 1: Use online converter');
  console.log('  - Go to: https://cloudconvert.com/png-to-icns');
  console.log('  - Upload: src-tauri/icons/128x128@2x.png (256x256)');
  console.log('  - Download and save as: src-tauri/icons/icon.icns');
  console.log('='.repeat(60));

  console.log('\n✓ Icon generation complete!');
  console.log('\nNext steps:');
  console.log('1. If ICO generation failed, convert icon-256.png to icon.ico online');
  console.log('2. Generate icon.icns using one of the methods above');
  console.log('3. Rebuild your Tauri app: npm run tauri build');
}

generateIcons().catch(err => {
  console.error('Error:', err.message);
  process.exit(1);
});
