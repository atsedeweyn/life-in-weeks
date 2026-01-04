/**
 * Life in Weeks Website - Interactive Scripts
 * Matrix rain effect and UI interactions
 */

// ========================================
// Matrix Rain Effect
// ========================================

(function initMatrix() {
    const canvas = document.getElementById('matrix-canvas');
    if (!canvas) return;
    
    const ctx = canvas.getContext('2d');
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    
    const matrix = "01アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン";
    const chars = matrix.split('');
    
    const fontSize = 14;
    const columns = canvas.width / fontSize;
    const drops = [];
    
    // Initialize drops
    for (let x = 0; x < columns; x++) {
        drops[x] = Math.random() * -100;
    }
    
    function draw() {
        // Semi-transparent black for trail effect
        ctx.fillStyle = 'rgba(10, 10, 10, 0.05)';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        
        ctx.fillStyle = '#00ff41';
        ctx.font = fontSize + 'px monospace';
        
        for (let i = 0; i < drops.length; i++) {
            const text = chars[Math.floor(Math.random() * chars.length)];
            const x = i * fontSize;
            const y = drops[i] * fontSize;
            
            // Gradient effect - brighter at top
            const opacity = Math.min(1, (canvas.height - y) / canvas.height);
            ctx.fillStyle = `rgba(0, 255, 65, ${opacity * 0.3})`;
            
            ctx.fillText(text, x, y);
            
            // Reset drop to top randomly
            if (y > canvas.height && Math.random() > 0.975) {
                drops[i] = 0;
            }
            
            drops[i]++;
        }
    }
    
    // Adjust canvas size on resize
    window.addEventListener('resize', () => {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
    });
    
    // Start animation
    setInterval(draw, 35);
})();

// ========================================
// Tab Switching
// ========================================

(function initTabs() {
    const tabButtons = document.querySelectorAll('.tab-btn');
    const tabContents = document.querySelectorAll('.tab-content');
    
    tabButtons.forEach(button => {
        button.addEventListener('click', () => {
            const targetTab = button.dataset.tab;
            
            // Remove active class from all buttons and contents
            tabButtons.forEach(btn => btn.classList.remove('active'));
            tabContents.forEach(content => content.classList.remove('active'));
            
            // Add active class to clicked button and corresponding content
            button.classList.add('active');
            document.getElementById(`${targetTab}-tab`).classList.add('active');
        });
    });
})();

// ========================================
// Platform Selector
// ========================================

(function initPlatformSelector() {
    const platformButtons = document.querySelectorAll('.platform-btn');
    const platformInstructions = document.querySelectorAll('.platform-instructions');
    
    platformButtons.forEach(button => {
        button.addEventListener('click', () => {
            const platform = button.dataset.platform;
            
            // Remove active class from all buttons and instructions
            platformButtons.forEach(btn => btn.classList.remove('active'));
            platformInstructions.forEach(inst => inst.classList.remove('active'));
            
            // Add active class to clicked button and corresponding instructions
            button.classList.add('active');
            document.querySelector(`.platform-instructions[data-platform="${platform}"]`).classList.add('active');
        });
    });
})();

// ========================================
// Smooth Scrolling
// ========================================

(function initSmoothScroll() {
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
})();

// ========================================
// Scroll Animations
// ========================================

(function initScrollAnimations() {
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
    
    // Observe feature cards and tutorial steps
    document.querySelectorAll('.feature-card, .tutorial-step, .note-card').forEach(el => {
        el.style.opacity = '0';
        el.style.transform = 'translateY(20px)';
        el.style.transition = 'opacity 0.6s ease, transform 0.6s ease';
        observer.observe(el);
    });
})();

// ========================================
// Typing Effect for Hero Title
// ========================================

(function initTypingEffect() {
    const heroTitle = document.querySelector('.hero-title');
    if (!heroTitle) return;
    
    const text = heroTitle.textContent;
    heroTitle.textContent = '';
    heroTitle.style.opacity = '1';
    
    let i = 0;
    function type() {
        if (i < text.length) {
            heroTitle.textContent += text.charAt(i);
            i++;
            setTimeout(type, 50);
        }
    }
    
    // Start typing after a short delay
    setTimeout(type, 500);
})();

// ========================================
// Copy Code Blocks (Optional Enhancement)
// ========================================

(function initCodeCopy() {
    const codeBlocks = document.querySelectorAll('.code-block');
    
    codeBlocks.forEach(block => {
        const button = document.createElement('button');
        button.textContent = 'Copy';
        button.className = 'copy-btn';
        button.style.cssText = `
            position: absolute;
            top: 8px;
            right: 8px;
            padding: 4px 12px;
            background: rgba(0, 255, 65, 0.1);
            border: 1px solid #00ff41;
            color: #00ff41;
            font-family: 'JetBrains Mono', monospace;
            font-size: 0.75rem;
            cursor: pointer;
            border-radius: 4px;
            transition: all 0.3s ease;
        `;
        
        button.addEventListener('mouseenter', () => {
            button.style.background = 'rgba(0, 255, 65, 0.2)';
        });
        
        button.addEventListener('mouseleave', () => {
            button.style.background = 'rgba(0, 255, 65, 0.1)';
        });
        
        button.addEventListener('click', () => {
            const code = block.querySelector('code').textContent;
            navigator.clipboard.writeText(code).then(() => {
                button.textContent = 'Copied!';
                setTimeout(() => {
                    button.textContent = 'Copy';
                }, 2000);
            });
        });
        
        block.style.position = 'relative';
        block.appendChild(button);
    });
})();

// ========================================
// Glitch Effect on Logo (Optional)
// ========================================

(function initGlitchEffect() {
    const logo = document.querySelector('.header h1');
    if (!logo) return;
    
    const originalText = logo.textContent;
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()';
    
    function glitch() {
        let glitched = '';
        for (let i = 0; i < originalText.length; i++) {
            if (Math.random() > 0.9) {
                glitched += chars[Math.floor(Math.random() * chars.length)];
            } else {
                glitched += originalText[i];
            }
        }
        logo.textContent = glitched;
        
        setTimeout(() => {
            logo.textContent = originalText;
        }, 100);
    }
    
    // Glitch on hover
    logo.addEventListener('mouseenter', () => {
        const interval = setInterval(glitch, 50);
        setTimeout(() => clearInterval(interval), 500);
    });
})();

// ========================================
// Stats Counter Animation
// ========================================

(function initStatsCounter() {
    const stats = document.querySelectorAll('.stat-value');
    
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting && !entry.target.dataset.animated) {
                entry.target.dataset.animated = 'true';
                animateValue(entry.target);
            }
        });
    }, { threshold: 0.5 });
    
    stats.forEach(stat => observer.observe(stat));
    
    function animateValue(element) {
        const target = parseInt(element.textContent.replace(/,/g, ''));
        const duration = 2000;
        const increment = target / (duration / 16);
        let current = 0;
        
        const timer = setInterval(() => {
            current += increment;
            if (current >= target) {
                element.textContent = target.toLocaleString();
                clearInterval(timer);
            } else {
                element.textContent = Math.floor(current).toLocaleString();
            }
        }, 16);
    }
})();

// ========================================
// Platform Detection & Direct Downloads
// ========================================

(async function initDownloads() {
    const REPO = 'atsedeweyn/life-in-weeks';
    const RELEASE_BASE = `https://github.com/${REPO}/releases/latest/download`;
    const RELEASE_PAGE = `https://github.com/${REPO}/releases/latest`;
    const RELEASE_API = `https://api.github.com/repos/${REPO}/releases/latest`;

    const fallbackCliUrls = {
        'windows': `${RELEASE_BASE}/liw-windows-amd64.exe`,
        'macos': `${RELEASE_BASE}/liw-macos-amd64`,
        'macos-arm': `${RELEASE_BASE}/liw-macos-arm64`,
        'linux': `${RELEASE_BASE}/liw-linux-amd64`,
        'unknown': RELEASE_PAGE
    };

    const fallbackGuiLinks = {
        'windows': { url: `${RELEASE_BASE}/liw-gui-windows.msi`, label: '.msi Installer' },
        'macos': { url: `${RELEASE_BASE}/liw-gui-macos.dmg`, label: '.dmg Installer' },
        'linux': { url: `${RELEASE_BASE}/liw-gui-linux.AppImage`, label: '.AppImage' }
    };

    const guiNamePattern = 'life[\\s_-]?in[\\s_-]?weeks';

    function pickAsset(assets, patterns) {
        for (const pattern of patterns) {
            const match = assets.find(asset => pattern.test(asset.name));
            if (match) {
                return match;
            }
        }
        return null;
    }

    async function loadReleaseAssets() {
        try {
            const res = await fetch(RELEASE_API, {
                headers: { 'Accept': 'application/vnd.github+json' }
            });
            if (!res.ok) {
                return null;
            }
            const data = await res.json();
            return Array.isArray(data.assets) ? data.assets : [];
        } catch (err) {
            return null;
        }
    }

    // Detect user's platform
    function detectPlatform() {
        const ua = navigator.userAgent.toLowerCase();
        const platform = navigator.platform?.toLowerCase() || '';

        if (ua.includes('win') || platform.includes('win')) {
            return 'windows';
        } else if (ua.includes('mac') || platform.includes('mac')) {
            // Check for Apple Silicon
            const isARM = ua.includes('arm') ||
                         (navigator.userAgentData?.platform === 'macOS' &&
                          navigator.userAgentData?.architecture === 'arm');
            return isARM ? 'macos-arm' : 'macos';
        } else if (ua.includes('linux') || platform.includes('linux')) {
            return 'linux';
        }
        return 'unknown';
    }

    const platform = detectPlatform();

    const platformNames = {
        'windows': 'Windows',
        'macos': 'macOS (Intel)',
        'macos-arm': 'macOS (Apple Silicon)',
        'linux': 'Linux'
    };

    let cliUrls = { ...fallbackCliUrls };
    let guiLinks = {
        'windows': { ...fallbackGuiLinks.windows },
        'macos': { ...fallbackGuiLinks.macos },
        'linux': { ...fallbackGuiLinks.linux }
    };

    const assets = await loadReleaseAssets();
    if (assets) {
        const cliAssetMap = {
            'windows': pickAsset(assets, [/^liw-windows-amd64\.exe$/i]),
            'macos': pickAsset(assets, [/^liw-macos-amd64$/i]),
            'macos-arm': pickAsset(assets, [/^liw-macos-arm64$/i]),
            'linux': pickAsset(assets, [/^liw-linux-amd64$/i])
        };

        Object.entries(cliAssetMap).forEach(([key, asset]) => {
            if (asset && asset.browser_download_url) {
                cliUrls[key] = asset.browser_download_url;
            }
        });

        const guiWin = pickAsset(assets, [
            /^liw-gui-windows\.exe$/i,
            /^liw-gui-windows\.msi$/i,
            new RegExp(`^${guiNamePattern}.*\\.exe$`, 'i'),
            new RegExp(`^${guiNamePattern}.*\\.msi$`, 'i')
        ]);
        if (guiWin && guiWin.browser_download_url) {
            guiLinks.windows.url = guiWin.browser_download_url;
            guiLinks.windows.label = guiWin.name.toLowerCase().endsWith('.exe') ? '.exe Installer' : '.msi Installer';
        }
        // If not found, keep fallback direct download URL (already set above)

        const guiMac = pickAsset(assets, [
            /^liw-gui-macos\.dmg$/i,
            /^liw-gui-macos\.app\.tar\.gz$/i,
            /^liw-gui-macos\.app\.zip$/i,
            new RegExp(`^${guiNamePattern}.*\\.dmg$`, 'i'),
            new RegExp(`^${guiNamePattern}.*\\.app\\.tar\\.gz$`, 'i'),
            new RegExp(`^${guiNamePattern}.*\\.app\\.zip$`, 'i')
        ]);
        if (guiMac && guiMac.browser_download_url) {
            guiLinks.macos.url = guiMac.browser_download_url;
            const macName = guiMac.name.toLowerCase();
            if (macName.endsWith('.dmg')) {
                guiLinks.macos.label = '.dmg Installer';
            } else if (macName.endsWith('.app.tar.gz')) {
                guiLinks.macos.label = '.app Bundle (.tar.gz)';
            } else if (macName.endsWith('.app.zip')) {
                guiLinks.macos.label = '.app Bundle (.zip)';
            } else {
                guiLinks.macos.label = 'macOS Bundle';
            }
        }
        // If not found, keep fallback direct download URL (already set above)

        const guiLinux = pickAsset(assets, [
            /^liw-gui-linux\.AppImage$/i,
            /^liw-gui-linux\.deb$/i,
            /^liw-gui-linux\.rpm$/i,
            new RegExp(`^${guiNamePattern}.*\\.AppImage$`, 'i'),
            new RegExp(`^${guiNamePattern}.*\\.deb$`, 'i'),
            new RegExp(`^${guiNamePattern}.*\\.rpm$`, 'i')
        ]);
        if (guiLinux && guiLinux.browser_download_url) {
            guiLinks.linux.url = guiLinux.browser_download_url;
            const linuxName = guiLinux.name.toLowerCase();
            if (linuxName.endsWith('.deb')) {
                guiLinks.linux.label = '.deb Package';
            } else if (linuxName.endsWith('.rpm')) {
                guiLinks.linux.label = '.rpm Package';
            } else {
                guiLinks.linux.label = '.AppImage';
            }
        }
        // If not found, keep fallback direct download URL (already set above)
    }

    // Update CLI download button
    const cliBtn = document.getElementById('cli-download-btn');
    if (cliBtn && platform !== 'unknown') {
        cliBtn.href = cliUrls[platform];
        cliBtn.textContent = `Download for ${platformNames[platform]}`;
        cliBtn.removeAttribute('target');
    }

    // Auto-select user's platform in the platform selector
    const platformMap = {
        'windows': 'windows',
        'macos': 'macos',
        'macos-arm': 'macos',
        'linux': 'linux'
    };

    const platformKey = platformMap[platform];
    if (platformKey) {
        const platformBtn = document.querySelector(`.platform-btn[data-platform="${platformKey}"]`);
        const platformInst = document.querySelector(`.platform-instructions[data-platform="${platformKey}"]`);

        if (platformBtn && platformInst) {
            document.querySelectorAll('.platform-btn').forEach(btn => btn.classList.remove('active'));
            document.querySelectorAll('.platform-instructions').forEach(inst => inst.classList.remove('active'));
            platformBtn.classList.add('active');
            platformInst.classList.add('active');
        }
    }

    // Highlight recommended GUI download card
    const guiCardMap = {
        'windows': 'gui-windows',
        'macos': 'gui-macos',
        'macos-arm': 'gui-macos',
        'linux': 'gui-linux'
    };

    const cardId = guiCardMap[platform];
    if (cardId) {
        const card = document.getElementById(cardId);
        if (card) {
            card.classList.add('recommended');
        }
    }

    function updateGuiCard(card, info) {
        if (!card) {
            return;
        }
        card.href = info.url;
        const label = card.querySelector('p');
        if (label && info.label) {
            label.textContent = info.label;
        }
        // Check if it's a direct download URL (not the releases page)
        if (info.url && info.url.includes('/releases/download/')) {
            // Direct download - remove target and add download attribute
            card.removeAttribute('target');
            // Extract filename from URL for download attribute
            const filename = info.url.split('/').pop().split('?')[0];
            card.setAttribute('download', filename);
        } else if (info.url === RELEASE_PAGE) {
            // If fallback to release page, open in new tab
            card.setAttribute('target', '_blank');
            card.removeAttribute('download');
        } else {
            // Any other URL (like browser_download_url from API)
            card.removeAttribute('target');
            const filename = info.url.split('/').pop().split('?')[0];
            card.setAttribute('download', filename);
        }
    }

    // Update GUI cards with direct download links
    updateGuiCard(document.getElementById('gui-windows'), guiLinks.windows);
    updateGuiCard(document.getElementById('gui-macos'), guiLinks.macos);
    updateGuiCard(document.getElementById('gui-linux'), guiLinks.linux);

    // Show support button after download clicks
    function showSupportAfterDownload() {
        // Check if we've already shown it in this session
        if (sessionStorage.getItem('supportShown')) {
            return;
        }
        
        // Create support message element
        const supportMessage = document.createElement('div');
        supportMessage.className = 'support-message';
        supportMessage.innerHTML = `
            <div class="support-message-content">
                <p class="support-message-text">Enjoying Life in Weeks? Consider supporting the project!</p>
                <a href="https://buymeacoffee.com/karpathy" target="_blank" class="support-btn support-btn-inline">
                    <span class="support-icon">☕</span>
                    <span>Buy me a coffee</span>
                </a>
                <button class="support-close" aria-label="Close">×</button>
            </div>
        `;
        
        // Add to page
        document.body.appendChild(supportMessage);
        
        // Animate in
        setTimeout(() => {
            supportMessage.classList.add('show');
        }, 100);
        
        // Close button handler
        const closeBtn = supportMessage.querySelector('.support-close');
        closeBtn.addEventListener('click', () => {
            supportMessage.classList.remove('show');
            setTimeout(() => {
                supportMessage.remove();
            }, 300);
        });
        
        // Auto-hide after 10 seconds
        setTimeout(() => {
            if (supportMessage.parentNode) {
                supportMessage.classList.remove('show');
                setTimeout(() => {
                    supportMessage.remove();
                }, 300);
            }
        }, 10000);
        
        // Mark as shown in this session
        sessionStorage.setItem('supportShown', 'true');
    }

    // Track download clicks
    const downloadElements = [
        document.getElementById('cli-download-btn'),
        document.getElementById('gui-windows'),
        document.getElementById('gui-macos'),
        document.getElementById('gui-linux')
    ];

    downloadElements.forEach(element => {
        if (element) {
            element.addEventListener('click', () => {
                // Small delay to ensure download starts
                setTimeout(showSupportAfterDownload, 500);
            });
        }
    });
})();

// ========================================
// Console Easter Egg
// ========================================

(function initEasterEgg() {
    const style = `
        color: #00ff41;
        font-family: 'JetBrains Mono', monospace;
        font-size: 14px;
        text-shadow: 0 0 10px rgba(0, 255, 65, 0.5);
    `;
    
    console.log('%cLife in Weeks', style);
    console.log('%cVisualize your time. Make it count.', 'color: #666; font-family: monospace;');
    console.log('%chttps://github.com/atsedeweyn/life-in-weeks', 'color: #00ff41; font-family: monospace;');
    
    // Easter egg command
    window.addEventListener('keydown', (e) => {
        if (e.ctrlKey && e.shiftKey && e.key === 'L') {
            console.log('%cYou found the secret! 🎉', 'color: #00ff41; font-size: 20px; font-weight: bold;');
        }
    });
})();
