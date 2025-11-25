# Gazelle Landing Page

A professional single-page website for hosting Gazelle CLI downloads.

## Structure

```
web/
├── index.html              # Main landing page
├── styles.css              # All styling and responsive design
├── script.js               # Interactive functionality
├── releases/               # Binary downloads directory
│   ├── gazelle-windows-x64.exe
│   ├── gazelle-macos-universal
│   ├── gazelle-linux-x64
│   ├── checksums.txt
│   └── README.md
└── README.md              # This file
```

## Features

### 🎨 Design
- Clean, modern design with Gazelle branding
- Responsive layout for desktop, tablet, and mobile
- Professional color scheme with Go blue (#00ADD8) accents
- Smooth animations and hover effects
- Inter font family for excellent readability

### 📱 User Experience
- Platform detection and automatic recommendations
- One-click downloads for Windows, macOS, and Linux
- Copy-to-clipboard for code examples
- Smooth scrolling navigation
- Loading states and user feedback
- Mobile-first responsive design

### 🔧 Functionality
- Automatic platform detection (Windows/macOS/Linux)
- Download tracking and analytics ready
- Code example copying
- Notification system for user feedback
- Service worker support for offline access
- Header scroll effects and animations

### 🚀 Performance
- Optimized CSS with minimal dependencies
- Progressive enhancement
- Lazy loading animations
- Fast load times with efficient asset delivery

## Deployment

### Static Hosting
The entire `web/` directory can be deployed to any static hosting service:

- **GitHub Pages**: Push to `gh-pages` branch
- **Netlify**: Drag and drop the `web/` folder
- **Vercel**: Import from GitHub repository
- **AWS S3**: Upload as static website
- **Nginx**: Serve as static files

### Building Releases
Use the build script to create binaries for all platforms:

```bash
# Build all platform binaries
./scripts/build-releases.sh

# Or build with specific version
./scripts/build-releases.sh v1.0.0
```

### Local Development
Serve locally for development:

```bash
# Python
cd web && python -m http.server 8000

# Node.js (if you have serve installed)
cd web && npx serve

# Go (from project root)
go run pkg/main.go serve-web
```

## Customization

### Branding
- Update colors in `styles.css` (search for `#00ADD8`)
- Replace logo/icons in HTML
- Modify hero section content

### Download Links
Update the `downloads` object in `script.js`:

```javascript
const downloads = {
    windows: {
        url: 'releases/gazelle-windows-x64.exe',
        filename: 'gz.exe',
        size: '8.2 MB'
    },
    // ... other platforms
};
```

### Analytics
Add tracking code before `</head>`:

```html
<!-- Google Analytics -->
<script async src="https://www.googletagmanager.com/gtag/js?id=GA_TRACKING_ID"></script>
<script>
    window.dataLayer = window.dataLayer || [];
    function gtag(){dataLayer.push(arguments);}
    gtag('js', new Date());
    gtag('config', 'GA_TRACKING_ID');
</script>
```

## Browser Support

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Mobile browsers (iOS Safari, Chrome Mobile)

## Performance

- First Contentful Paint: <1.5s
- Largest Contentful Paint: <2.5s
- Cumulative Layout Shift: <0.1
- First Input Delay: <100ms

## Security

- No external dependencies (except Google Fonts)
- CSP-ready design
- HTTPS recommended for production
- File integrity verification via checksums