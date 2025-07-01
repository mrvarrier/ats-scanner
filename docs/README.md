# ATS Scanner Landing Page

This directory contains the static website for ATS Scanner, hosted on GitHub Pages.

## 🌐 Live Site

Visit the live site at: https://mrvarrier.github.io/ats-scanner

## 📁 Structure

- `index.html` - Main landing page
- `404.html` - Custom 404 error page  
- `_config.yml` - Jekyll configuration for GitHub Pages
- `robots.txt` - Search engine indexing rules
- `README.md` - This file

## 🚀 Features

- **Responsive Design** - Mobile-first approach with clean layouts
- **Platform Detection** - Smart download buttons that detect user's OS
- **Modern UI/UX** - Clean, professional design with smooth animations
- **SEO Optimized** - Proper meta tags and semantic HTML
- **Performance** - Lightweight, fast-loading static site
- **Accessibility** - WCAG compliant design patterns

## 🛠 Technology Stack

- Pure HTML/CSS/JavaScript (no frameworks)
- CSS Grid & Flexbox for layouts
- CSS custom properties for theming
- Intersection Observer API for animations
- GitHub Pages for hosting

## 🎨 Design System

### Colors
- Primary Blue: `#3B82F6`
- Primary Blue Dark: `#2563EB` 
- Light Blue: `#EFF6FF`
- Gray Scale: `#F9FAFB` to `#111827`
- Success Green: `#10B981`

### Typography
- System font stack for performance
- Font weights: 500, 600, 700, 800
- Responsive sizing using `clamp()`

### Spacing
- 4px base unit system
- Consistent padding/margin scale
- Container max-width: 1200px

## 📱 Responsive Breakpoints

- Mobile: < 768px
- Tablet: 768px - 1024px  
- Desktop: > 1024px

## 🔧 Local Development

To test locally:

1. Clone the repository
2. Navigate to the `docs` folder
3. Serve with any static file server:
   ```bash
   # Using Python
   python -m http.server 8000
   
   # Using Node.js
   npx serve .
   
   # Using PHP
   php -S localhost:8000
   ```
4. Open http://localhost:8000

## 📊 Analytics

The site includes download tracking via JavaScript console logs. For production analytics, integrate with your preferred service (Google Analytics, Plausible, etc.).

## 🚀 Deployment

The site automatically deploys to GitHub Pages when changes are pushed to the `main` branch. GitHub Pages builds and serves the content from the `/docs` folder.

## 📝 Content Updates

To update download links or version numbers:

1. Edit the URLs in `index.html`
2. Update version numbers in the download cards
3. Commit and push changes
4. GitHub Pages will automatically rebuild

## 🤝 Contributing

To improve the landing page:

1. Fork the repository
2. Make changes in the `docs` folder
3. Test locally
4. Submit a pull request

## 📄 License

This landing page is part of the ATS Scanner project and is released under the MIT License.