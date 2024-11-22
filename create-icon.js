import { createCanvas } from 'canvas';
import fs from 'fs';
import path from 'path';

const sizes = [32, 128, 256]; // 256 will be used for @2x version of 128

function createIcon(size) {
    const canvas = createCanvas(size, size);
    const ctx = canvas.getContext('2d');

    // Background
    ctx.fillStyle = '#2C2C2C';
    ctx.fillRect(0, 0, size, size);

    // Simple bass clef-inspired design
    ctx.strokeStyle = '#FFD700';
    ctx.lineWidth = size / 16;
    ctx.beginPath();
    ctx.arc(size * 0.5, size * 0.4, size * 0.3, 0, Math.PI * 2);
    ctx.stroke();

    // Dots
    ctx.fillStyle = '#FFD700';
    ctx.beginPath();
    ctx.arc(size * 0.7, size * 0.6, size * 0.08, 0, Math.PI * 2);
    ctx.fill();
    ctx.beginPath();
    ctx.arc(size * 0.7, size * 0.8, size * 0.08, 0, Math.PI * 2);
    ctx.fill();

    return canvas;
}

// Create icons directory if it doesn't exist
const iconsDir = path.join(process.cwd(), 'src-tauri', 'icons');
if (!fs.existsSync(iconsDir)) {
    fs.mkdirSync(iconsDir, { recursive: true });
}

// Generate icons
sizes.forEach(size => {
    const canvas = createIcon(size);
    const fileName = size === 256 ? '128x128@2x.png' : `${size}x${size}.png`;
    const buffer = canvas.toBuffer('image/png');
    fs.writeFileSync(path.join(iconsDir, fileName), buffer);
});

// Create icns and ico files
console.log('Icons generated successfully!');
