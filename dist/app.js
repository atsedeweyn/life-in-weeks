/**
 * Life in Weeks - Frontend Application
 * 
 * Interfaces with Tauri backend for wallpaper generation.
 */

// State
let currentMode = 'year-end';
let currentTheme = 'dark';
let hasPreview = false;

// DOM Elements
const elements = {
    modeTabs: document.querySelectorAll('.mode-tab'),
    themeBtns: document.querySelectorAll('.theme-btn'),
    dobInput: document.getElementById('dob'),
    lifespanInput: document.getElementById('lifespan'),
    monthsInput: document.getElementById('months'),
    widthInput: document.getElementById('width'),
    heightInput: document.getElementById('height'),
    detectResolutionBtn: document.getElementById('detect-resolution'),
    generateBtn: document.getElementById('generate-btn'),
    setWallpaperBtn: document.getElementById('set-wallpaper-btn'),
    scheduleToggle: document.getElementById('schedule-toggle'),
    previewPlaceholder: document.getElementById('preview-placeholder'),
    previewImage: document.getElementById('preview-image'),
    previewTitle: document.getElementById('preview-title'),
    previewSubtitle: document.getElementById('preview-subtitle'),
    statElapsed: document.getElementById('stat-elapsed'),
    statRemaining: document.getElementById('stat-remaining'),
    statPercent: document.getElementById('stat-percent'),
    toast: document.getElementById('toast'),
    toastMessage: document.getElementById('toast-message'),
    lifeOnlyElements: document.querySelectorAll('.life-only'),
    monthsOnlyElements: document.querySelectorAll('.months-only'),
};

// Tauri API (available via withGlobalTauri)
const { invoke } = window.__TAURI__.core;

/**
 * Initialize the application
 */
async function init() {
    setupEventListeners();
    await loadConfig();
    updateModeVisibility();
    detectScreenResolution();
}

/**
 * Set up event listeners
 */
function setupEventListeners() {
    // Mode tabs
    elements.modeTabs.forEach(tab => {
        tab.addEventListener('click', () => {
            elements.modeTabs.forEach(t => t.classList.remove('active'));
            tab.classList.add('active');
            currentMode = tab.dataset.mode;
            updateModeVisibility();
        });
    });

    // Theme buttons
    elements.themeBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            elements.themeBtns.forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            currentTheme = btn.dataset.theme;
        });
    });

    // Detect resolution button
    elements.detectResolutionBtn.addEventListener('click', detectScreenResolution);

    // Generate button
    elements.generateBtn.addEventListener('click', generatePreview);

    // Set wallpaper button
    elements.setWallpaperBtn.addEventListener('click', setWallpaper);

    // Schedule toggle
    elements.scheduleToggle.addEventListener('change', toggleSchedule);

    // Auto-generate on input change (debounced)
    const inputs = [
        elements.dobInput,
        elements.lifespanInput,
        elements.monthsInput,
    ];
    inputs.forEach(input => {
        input.addEventListener('change', () => {
            if (hasPreview) generatePreview();
        });
    });
}

/**
 * Update visibility of mode-specific inputs
 */
function updateModeVisibility() {
    const isLifeMode = currentMode === 'life';
    const isMonthsMode = currentMode === 'next-months';

    elements.lifeOnlyElements.forEach(el => {
        el.classList.toggle('hidden', !isLifeMode);
    });

    elements.monthsOnlyElements.forEach(el => {
        el.classList.toggle('hidden', !isMonthsMode);
    });
}

/**
 * Load configuration from backend
 */
async function loadConfig() {
    try {
        const config = await invoke('get_config');
        
        if (config.dob) {
            elements.dobInput.value = config.dob;
        }
        elements.lifespanInput.value = config.lifespan_years;
        elements.widthInput.value = config.screen_width;
        elements.heightInput.value = config.screen_height;
        elements.monthsInput.value = config.next_months;
        elements.scheduleToggle.checked = config.schedule_installed;

        // Set theme
        currentTheme = config.theme;
        elements.themeBtns.forEach(btn => {
            btn.classList.toggle('active', btn.dataset.theme === currentTheme);
        });

        // Set mode
        if (config.default_mode) {
            currentMode = config.default_mode;
            elements.modeTabs.forEach(tab => {
                tab.classList.toggle('active', tab.dataset.mode === currentMode);
            });
            updateModeVisibility();
        }
    } catch (error) {
        console.log('Could not load config, using defaults:', error);
    }
}

/**
 * Detect screen resolution
 */
function detectScreenResolution() {
    const width = window.screen.width * window.devicePixelRatio;
    const height = window.screen.height * window.devicePixelRatio;
    elements.widthInput.value = Math.round(width);
    elements.heightInput.value = Math.round(height);
    showToast('Screen resolution detected', 'success');
}

/**
 * Generate a preview image
 */
async function generatePreview() {
    const btn = elements.generateBtn;
    btn.classList.add('loading');
    btn.disabled = true;

    try {
        const request = buildRequest();
        const response = await invoke('generate_preview', { request });

        // Update preview image
        elements.previewImage.src = `data:image/png;base64,${response.image_base64}`;
        elements.previewImage.classList.remove('hidden');
        elements.previewPlaceholder.classList.add('hidden');

        // Update title and subtitle
        elements.previewTitle.textContent = response.title;
        elements.previewSubtitle.textContent = response.subtitle;

        // Update stats
        elements.statElapsed.textContent = formatNumber(response.elapsed_weeks);
        elements.statRemaining.textContent = formatNumber(response.remaining_weeks);
        
        const percent = response.total_weeks > 0 
            ? Math.round((response.elapsed_weeks / response.total_weeks) * 100) 
            : 0;
        elements.statPercent.textContent = `${percent}%`;

        // Enable set wallpaper button
        elements.setWallpaperBtn.disabled = false;
        hasPreview = true;

        showToast('Preview generated!', 'success');
    } catch (error) {
        showToast(`Error: ${error}`, 'error');
        console.error('Generate preview error:', error);
    } finally {
        btn.classList.remove('loading');
        btn.disabled = false;
    }
}

/**
 * Set the generated image as wallpaper
 */
async function setWallpaper() {
    const btn = elements.setWallpaperBtn;
    btn.classList.add('loading');
    btn.disabled = true;

    try {
        const request = buildRequest();
        const message = await invoke('set_wallpaper_cmd', { request });
        
        // Save config
        await saveConfig();

        showToast('Wallpaper set successfully!', 'success');
    } catch (error) {
        showToast(`Error: ${error}`, 'error');
        console.error('Set wallpaper error:', error);
    } finally {
        btn.classList.remove('loading');
        btn.disabled = false;
    }
}

/**
 * Toggle automatic schedule
 */
async function toggleSchedule() {
    const enabled = elements.scheduleToggle.checked;

    try {
        const message = await invoke('toggle_schedule', { enabled });
        showToast(message, 'success');
    } catch (error) {
        elements.scheduleToggle.checked = !enabled;
        showToast(`Error: ${error}`, 'error');
        console.error('Toggle schedule error:', error);
    }
}

/**
 * Save current configuration
 */
async function saveConfig() {
    try {
        await invoke('save_config', {
            dob: elements.dobInput.value || null,
            lifespan: parseInt(elements.lifespanInput.value) || null,
            theme: currentTheme,
            width: parseInt(elements.widthInput.value) || null,
            height: parseInt(elements.heightInput.value) || null,
            defaultMode: currentMode,
            months: parseInt(elements.monthsInput.value) || null,
        });
    } catch (error) {
        console.error('Save config error:', error);
    }
}

/**
 * Build a request object from current state
 */
function buildRequest() {
    return {
        mode: currentMode,
        dob: elements.dobInput.value || null,
        lifespan: parseInt(elements.lifespanInput.value) || null,
        months: parseInt(elements.monthsInput.value) || null,
        theme: currentTheme,
        width: parseInt(elements.widthInput.value) || null,
        height: parseInt(elements.heightInput.value) || null,
    };
}

/**
 * Format a number with commas
 */
function formatNumber(num) {
    return num.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',');
}

/**
 * Show a toast notification
 */
function showToast(message, type = 'info') {
    elements.toastMessage.textContent = message;
    elements.toast.className = `toast show ${type}`;
    
    setTimeout(() => {
        elements.toast.classList.remove('show');
    }, 3000);
}

// Initialize when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}
