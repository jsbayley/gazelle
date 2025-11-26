# Gazelle Documentation Design System

## Color Consolidation (v0.0.5)

The Gazelle documentation website has been thoroughly redesigned with a consolidated, professional color system that ensures excellent readability, accessibility, and visual consistency across all elements.

## 🎨 Color Palette

### Primary Colors
```css
--accent-primary: #3182ce      /* Professional blue */
--accent-primary-hover: #2c5282
--accent-secondary: #38a169    /* Success green */
--accent-warning: #d69e2e      /* Warning amber */
--accent-danger: #e53e3e       /* Error red */
```

### Status Colors
```css
--status-success: #38a169
--status-warning: #d69e2e  
--status-error: #e53e3e
--status-info: #3182ce
```

### Text Hierarchy
```css
--text-primary: #1a202c       /* High contrast headings */
--text-secondary: #2d3748     /* Body text */
--text-tertiary: #4a5568      /* Supporting text */
```

### Background System
```css
--bg-primary: #ffffff         /* Main background */
--bg-secondary: #f7fafc       /* Card backgrounds */
--bg-tertiary: #edf2f7        /* Subtle accents */
--code-bg: #f7fafc            /* Code blocks */
```

### Interactive States
```css
--hover-bg: #edf2f7          /* Hover backgrounds */
--active-bg: #e2e8f0         /* Active states */
--focus-ring: #63b3ed        /* Focus indicators */
```

## 🌙 Dark Theme Support

All colors automatically adapt to dark themes with carefully chosen variants that maintain contrast ratios and visual hierarchy:

- Primary accent shifts to lighter blue (`#63b3ed`)
- Backgrounds use professional dark greys
- Text maintains excellent contrast ratios
- Interactive states provide clear feedback

## ✅ Key Improvements

### Before Consolidation
- **20+ hardcoded colors** scattered throughout CSS
- **Inconsistent color usage** across similar elements  
- **Poor accessibility** with low contrast ratios
- **Maintenance burden** with repeated color definitions
- **Mixed color schemes** (purple, blue, green, orange)

### After Consolidation  
- **Unified color system** with semantic variables
- **Consistent visual hierarchy** across all elements
- **WCAG compliant contrast ratios** for accessibility
- **Easy maintenance** with centralized color definitions
- **Professional blue-grey palette** with strategic accent colors

## 🎯 Usage Guidelines

### Primary Actions
Use `--accent-primary` for main CTAs, active navigation, and primary interactive elements.

### Status Communication
- Success states: `--status-success` 
- Warnings: `--status-warning`
- Errors: `--status-error`
- Information: `--status-info`

### Text Hierarchy
- Headings: `--text-primary`
- Body copy: `--text-secondary` 
- Supporting text: `--text-tertiary`

### Interactive Elements
- Hover states: `--hover-bg`
- Active states: `--active-bg`
- Focus rings: `--focus-ring`

## 🚀 Performance Benefits

- **Consistent brand experience** across all documentation
- **Reduced CSS complexity** with centralized color management
- **Better maintainability** for future updates
- **Enhanced accessibility** with proper contrast ratios
- **Responsive design** that works across all devices

## 🔧 Technical Implementation

All colors are defined as CSS custom properties (variables) in the `:root` selector, with automatic dark theme variants using `@media (prefers-color-scheme: dark)` and explicit theme toggles via `[data-theme="dark"]` selectors.

The design system replaces over 50 hardcoded color instances with a semantic, maintainable system that ensures consistency and accessibility across the entire documentation experience.