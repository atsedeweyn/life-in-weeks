# Life in Weeks Website

A minimal, tech/Matrix-themed website for the Life in Weeks project.

## Features

- **Matrix Rain Background** - Animated falling characters
- **Responsive Design** - Works on all devices
- **Interactive Tutorials** - Step-by-step guides for CLI and GUI
- **Platform-Specific Instructions** - Windows, macOS, and Linux
- **Smooth Animations** - Scroll-triggered animations and effects

## Local Development

Simply open `index.html` in a web browser. No build process required!

```bash
cd website
# Open in browser
open index.html  # macOS
xdg-open index.html  # Linux
start index.html  # Windows
```

Or use a simple HTTP server:

```bash
# Python 3
python3 -m http.server 8000

# Node.js (with http-server)
npx http-server -p 8000

# PHP
php -S localhost:8000
```

Then visit `http://localhost:8000`

## Deployment

### GitHub Pages

1. Push the `website/` folder contents to a `gh-pages` branch
2. Enable GitHub Pages in repository settings
3. Select the `gh-pages` branch as source

Or use GitHub Actions:

```yaml
name: Deploy Website
on:
  push:
    branches: [main]
    paths: ['website/**']

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./website
```

### Netlify

1. Drag and drop the `website/` folder to Netlify
2. Or connect your GitHub repo and set publish directory to `website`

### Vercel

1. Install Vercel CLI: `npm i -g vercel`
2. Run `vercel` in the `website/` directory
3. Follow the prompts

## Customization

### Colors

Edit `styles.css` and modify the CSS variables in `:root`:

```css
:root {
    --matrix-green: #00ff41;
    --bg-primary: #0a0a0a;
    /* ... */
}
```

### Matrix Characters

Edit `script.js` and modify the `matrix` variable:

```javascript
const matrix = "01アイウエオカキクケコ...";
```

### Content

All content is in `index.html`. Edit sections as needed.

## Browser Support

- Chrome/Edge (latest)
- Firefox (latest)
- Safari (latest)
- Mobile browsers

## License

MIT License - same as the main project
