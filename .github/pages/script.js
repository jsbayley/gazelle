// Download functionality
const downloads = {
    windows: {
        url: 'releases/gz.win-x64.exe',
        filename: 'gz.exe'
    },
    'macos-intel': {
        url: 'releases/gz.osx-x64',
        filename: 'gz'
    },
    'macos-arm64': {
        url: 'releases/gz.osx-arm64',
        filename: 'gz'
    },
    linux: {
        url: 'releases/gz.linux-x64',
        filename: 'gz'
    },
    'linux-arm64': {
        url: 'releases/gz.linux-arm64',
        filename: 'gz'
    }
};

// Download file function
function downloadFile(platform) {
    const download = downloads[platform];
    if (!download) return;
    
    const button = document.querySelector(`[data-platform="${platform}"] .btn-download`);
    const originalText = button.innerHTML;
    
    // Show loading state
    button.classList.add('downloading');
    button.innerHTML = '<span class="download-icon">⏳</span> Downloading...';
    button.disabled = true;
    
    // Track download event
    if (typeof gtag !== 'undefined') {
        gtag('event', 'download', {
            'event_category': 'engagement',
            'event_label': platform
        });
    }
    
    // Simulate download process (replace with actual download logic)
    setTimeout(() => {
        // Create download link
        const link = document.createElement('a');
        link.href = download.url;
        link.download = download.filename;
        
        // Check if file exists, otherwise show placeholder message
        fetch(download.url, { method: 'HEAD' })
            .then(response => {
                if (response.ok) {
                    document.body.appendChild(link);
                    link.click();
                    document.body.removeChild(link);
                } else {
                    showNotification(`Download for ${platform} will be available soon. Please check back later or build from source.`, 'info');
                }
            })
            .catch(() => {
                showNotification(`Download for ${platform} will be available soon. Please check back later or build from source.`, 'info');
            })
            .finally(() => {
                // Reset button state
                button.classList.remove('downloading');
                button.innerHTML = originalText;
                button.disabled = false;
            });
    }, 1000);
}

// Copy code functionality
function copyCode(button) {
    const codeBlock = button.closest('.code-block').querySelector('code');
    const text = codeBlock.textContent;
    
    navigator.clipboard.writeText(text).then(() => {
        const originalText = button.textContent;
        button.classList.add('copied');
        button.textContent = 'Copied!';
        
        setTimeout(() => {
            button.classList.remove('copied');
            button.textContent = originalText;
        }, 2000);
    }).catch(err => {
        console.error('Failed to copy text: ', err);
        showNotification('Failed to copy code to clipboard', 'error');
    });
}

// Notification system
function showNotification(message, type = 'info') {
    const notification = document.createElement('div');
    notification.className = `notification notification-${type}`;
    notification.innerHTML = `
        <div class="notification-content">
            <span class="notification-icon">${type === 'error' ? '❌' : 'ℹ️'}</span>
            <span class="notification-message">${message}</span>
            <button class="notification-close" onclick="this.parentElement.parentElement.remove()">×</button>
        </div>
    `;
    
    // Add notification styles if not already present
    if (!document.getElementById('notification-styles')) {
        const styles = document.createElement('style');
        styles.id = 'notification-styles';
        styles.textContent = `
            .notification {
                position: fixed;
                top: 100px;
                right: 20px;
                background: white;
                border: 1px solid #e2e8f0;
                border-radius: 8px;
                box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
                z-index: 1000;
                max-width: 400px;
                animation: slideIn 0.3s ease;
            }
            
            .notification-info {
                border-left: 4px solid #4299e1;
            }
            
            .notification-error {
                border-left: 4px solid #f56565;
            }
            
            .notification-content {
                padding: 1rem;
                display: flex;
                align-items: center;
                gap: 0.75rem;
            }
            
            .notification-message {
                flex: 1;
                font-size: 0.875rem;
                color: #4a5568;
            }
            
            .notification-close {
                background: none;
                border: none;
                font-size: 1.25rem;
                cursor: pointer;
                color: #a0aec0;
                padding: 0;
                width: 24px;
                height: 24px;
                display: flex;
                align-items: center;
                justify-content: center;
            }
            
            .notification-close:hover {
                color: #4a5568;
            }
            
            @keyframes slideIn {
                from {
                    transform: translateX(100%);
                    opacity: 0;
                }
                to {
                    transform: translateX(0);
                    opacity: 1;
                }
            }
        `;
        document.head.appendChild(styles);
    }
    
    document.body.appendChild(notification);
    
    // Auto remove after 5 seconds
    setTimeout(() => {
        if (notification.parentElement) {
            notification.remove();
        }
    }, 5000);
}

// Smooth scrolling for navigation links
function initSmoothScrolling() {
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                const headerOffset = 100;
                const elementPosition = target.getBoundingClientRect().top;
                const offsetPosition = elementPosition + window.pageYOffset - headerOffset;

                window.scrollTo({
                    top: offsetPosition,
                    behavior: 'smooth'
                });
            }
        });
    });
}

// Platform detection and highlighting
function detectPlatform() {
    const userAgent = navigator.userAgent.toLowerCase();
    let platform = 'linux'; // default
    
    if (userAgent.includes('windows')) {
        platform = 'windows';
    } else if (userAgent.includes('mac')) {
        // More robust Apple Silicon detection
        // Check for ARM architecture indicators
        if (userAgent.includes('arm') || 
            navigator.platform === 'MacIntel' && navigator.maxTouchPoints > 1) {
            // Apple Silicon Macs report MacIntel but have touch points
            platform = 'macos-arm64';
        } else {
            // Intel Macs
            platform = 'macos-intel';
        }
    }
    
    // Highlight the detected platform
    const platformCard = document.querySelector(`[data-platform="${platform}"]`);
    if (platformCard) {
        platformCard.style.order = '-1';
        platformCard.classList.add('recommended');
        
        // Theme-aware background highlighting
        const isDark = document.documentElement.getAttribute('data-theme') === 'dark';
        const highlightBg = isDark 
            ? 'linear-gradient(135deg, rgba(0, 173, 216, 0.1) 0%, rgba(0, 153, 199, 0.1) 100%)'
            : 'linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%)';
        
        platformCard.style.background = highlightBg;
        
        // Add "recommended" badge
        const badge = document.createElement('div');
        badge.className = 'recommended-badge';
        badge.textContent = 'RECOMMENDED';
        platformCard.style.position = 'relative';
        platformCard.appendChild(badge);
        
        console.log(`Detected platform: ${platform}`);
    }
}

// Initialize file size display with dynamic fetching
function updateFileSizes() {
    Object.keys(downloads).forEach(platform => {
        const sizeElement = document.getElementById(`${platform}-size`);
        if (sizeElement) {
            // Show loading state
            sizeElement.textContent = 'Loading...';
            
            // Fetch actual file size
            fetch(downloads[platform].url, { method: 'HEAD' })
                .then(response => {
                    if (response.ok) {
                        const sizeBytes = parseInt(response.headers.get('content-length'));
                        if (sizeBytes) {
                            // Check if content is compressed
                            const encoding = response.headers.get('content-encoding');
                            let sizeMB = (sizeBytes / (1024 * 1024)).toFixed(1);
                            
                            // If compressed, estimate uncompressed size (rough approximation)
                            if (encoding && encoding.includes('gzip')) {
                                sizeMB = (sizeBytes * 2.5 / (1024 * 1024)).toFixed(1); // Rough estimate
                            }
                            
                            sizeElement.textContent = `~${sizeMB} MB`;
                        } else {
                            sizeElement.textContent = '~?.? MB';
                        }
                    } else {
                        sizeElement.textContent = 'Not available';
                    }
                })
                .catch(() => {
                    sizeElement.textContent = 'Not available';
                });
        }
    });
}

// Test download availability
function testDownloadAvailability() {
    Object.keys(downloads).forEach(platform => {
        const download = downloads[platform];
        const button = document.querySelector(`[data-platform="${platform}"] .btn-download`);
        
        if (button) {
            fetch(download.url, { method: 'HEAD' })
                .then(response => {
                    if (response.ok) {
                        // File available - restore normal appearance
                        button.style.opacity = '1';
                        button.title = '';
                    } else {
                        // File not available - dim button
                        button.style.opacity = '0.7';
                        button.title = 'Download will be available soon';
                    }
                })
                .catch(() => {
                    // Error fetching - dim button
                    button.style.opacity = '0.7';
                    button.title = 'Download will be available soon';
                });
        }
    });
}

// Header scroll effect
function initHeaderScroll() {
    const header = document.querySelector('.header');
    let lastScrollY = window.scrollY;
    
    window.addEventListener('scroll', () => {
        const currentScrollY = window.scrollY;
        
        // Get current theme colors
        const isDark = document.documentElement.getAttribute('data-theme') === 'dark';
        const bgColor = isDark ? 'rgba(15, 20, 25, 0.98)' : 'rgba(255, 255, 255, 0.98)';
        const borderColor = isDark ? 'rgba(45, 55, 72, 0.8)' : 'rgba(226, 232, 240, 0.8)';
        
        if (currentScrollY > 100) {
            header.style.background = bgColor;
            header.style.backdropFilter = 'blur(20px)';
            header.style.borderBottom = `1px solid ${borderColor}`;
        } else {
            header.style.background = '';
            header.style.backdropFilter = '';
            header.style.borderBottom = '';
        }
        
        // Hide/show header on scroll
        if (currentScrollY > lastScrollY && currentScrollY > 200) {
            header.style.transform = 'translateY(-100%)';
        } else {
            header.style.transform = 'translateY(0)';
        }
        
        lastScrollY = currentScrollY;
    });
}

// Animate elements on scroll
function initScrollAnimations() {
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };
    
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.style.opacity = '1';
                entry.target.style.transform = 'translateY(0)';
            }
        });
    }, observerOptions);
    
    // Observe elements for animation
    document.querySelectorAll('.feature-card, .download-card, .doc-card').forEach(el => {
        el.style.opacity = '0';
        el.style.transform = 'translateY(20px)';
        el.style.transition = 'opacity 0.6s ease, transform 0.6s ease';
        observer.observe(el);
    });
}

// Theme management
function initTheme() {
    // Check for saved theme preference or default to system preference
    const savedTheme = localStorage.getItem('theme');
    const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    
    if (savedTheme) {
        document.documentElement.setAttribute('data-theme', savedTheme);
    } else if (systemPrefersDark) {
        document.documentElement.setAttribute('data-theme', 'dark');
    } else {
        document.documentElement.setAttribute('data-theme', 'light');
    }
    
    updateThemeIcon();
}

function toggleTheme() {
    const currentTheme = document.documentElement.getAttribute('data-theme');
    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
    
    document.documentElement.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);
    updateThemeIcon();
    updateHeaderColors();
    updateRecommendedStyling();
}

function updateRecommendedStyling() {
    const recommendedCard = document.querySelector('.recommended-badge')?.parentElement;
    if (recommendedCard) {
        const isDark = document.documentElement.getAttribute('data-theme') === 'dark';
        const highlightBg = isDark 
            ? 'linear-gradient(135deg, rgba(0, 173, 216, 0.1) 0%, rgba(0, 153, 199, 0.1) 100%)'
            : 'linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%)';
        
        recommendedCard.style.background = highlightBg;
    }
}

function updateThemeIcon() {
    const themeIcon = document.querySelector('.theme-icon');
    const currentTheme = document.documentElement.getAttribute('data-theme');
    
    if (themeIcon) {
        themeIcon.textContent = currentTheme === 'dark' ? '☀️' : '🌙';
    }
}

function updateHeaderColors() {
    const header = document.querySelector('.header');
    const currentScrollY = window.scrollY;
    
    if (currentScrollY > 100) {
        // Get current theme colors
        const isDark = document.documentElement.getAttribute('data-theme') === 'dark';
        const bgColor = isDark ? 'rgba(15, 20, 25, 0.98)' : 'rgba(255, 255, 255, 0.98)';
        const borderColor = isDark ? 'rgba(45, 55, 72, 0.8)' : 'rgba(226, 232, 240, 0.8)';
        
        header.style.background = bgColor;
        header.style.borderBottom = `1px solid ${borderColor}`;
    }
}

// Mobile menu toggle
function toggleMobileMenu() {
    const nav = document.querySelector('.nav');
    const hamburger = document.querySelector('.hamburger-menu');
    
    nav.classList.toggle('mobile-open');
    hamburger.classList.toggle('active');
}

// Close mobile menu when clicking on a link
function closeMobileMenu() {
    const nav = document.querySelector('.nav');
    const hamburger = document.querySelector('.hamburger-menu');
    
    nav.classList.remove('mobile-open');
    hamburger.classList.remove('active');
}

// Listen for system theme changes
window.matchMedia('(prefers-color-scheme: dark)').addListener((e) => {
    if (!localStorage.getItem('theme')) {
        document.documentElement.setAttribute('data-theme', e.matches ? 'dark' : 'light');
        updateThemeIcon();
    }
});

// Initialize everything when DOM is loaded
document.addEventListener('DOMContentLoaded', function() {
    initTheme();
    initSmoothScrolling();
    detectPlatform();
    updateFileSizes();
    testDownloadAvailability();
    initHeaderScroll();
    initScrollAnimations();
    
    // Add mobile menu close functionality to nav links
    document.querySelectorAll('.nav-link').forEach(link => {
        link.addEventListener('click', closeMobileMenu);
    });
    
    // Close mobile menu when clicking outside
    document.addEventListener('click', function(e) {
        const nav = document.querySelector('.nav');
        const hamburger = document.querySelector('.hamburger-menu');
        
        if (nav.classList.contains('mobile-open') && 
            !nav.contains(e.target) && 
            !hamburger.contains(e.target)) {
            closeMobileMenu();
        }
    });
    
    // Add keyboard shortcuts
    document.addEventListener('keydown', function(e) {
        // Ctrl/Cmd + K to focus search (if implemented)
        if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
            e.preventDefault();
            // Focus search if available
        }
        
        // Escape to close notifications or mobile menu
        if (e.key === 'Escape') {
            document.querySelectorAll('.notification').forEach(n => n.remove());
            closeMobileMenu();
        }
    });
});

// Performance monitoring (optional)
if ('performance' in window) {
    window.addEventListener('load', function() {
        const loadTime = performance.now();
        if (loadTime > 3000) {
            console.warn('Page load time is slower than expected:', Math.round(loadTime), 'ms');
        }
    });
}

// Service Worker for offline support (progressive enhancement)
if ('serviceWorker' in navigator) {
    window.addEventListener('load', function() {
        navigator.serviceWorker.register('/sw.js').then(function(registration) {
            console.log('ServiceWorker registration successful');
        }).catch(function(err) {
            console.log('ServiceWorker registration failed');
        });
    });
}