<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  interface Settings {
    trigger_key: string;
    jump_key: string;
    crouch_key: string;
    fps: number;
    delay_ms: number;
    jump_hold_us: number;
    crouch_hold_ms: number;
    theme: string;
    language: string;
  }

  const TRANSLATIONS = {
    en: {
      status_inactive: 'INACTIVE',
      status_active: 'ACTIVE',
      hint_active: 'Press trigger key to superglide',
      hint_inactive: 'Click to activate',
      btn_start: 'START',
      btn_stop: 'STOP',
      section_fps: 'FRAMERATE',
      section_timing: 'TIMING',
      section_keys: 'KEY BINDINGS',
      label_delay: 'DELAY',
      label_jump_hold: 'JUMP HOLD',
      label_hold: 'HOLD TIME',
      desc_delay: 'Between jump and crouch',
      desc_jump_hold: 'Jump key press duration',
      desc_hold: 'Crouch hold duration',
      key_trigger: 'TRIGGER',
      key_jump: 'JUMP',
      key_crouch: 'CROUCH',
      desc_trigger: 'Activates the macro',
      desc_jump: 'Jump button in game',
      desc_crouch: 'Crouch button in game',
      press_any: 'PRESS KEY…',
      capturing_hint: 'Press any key…',
      triggered: 'TRIGGERED',
      tt_theme: 'Toggle theme',
      tt_minimize: 'Minimize to tray',
      tt_close: 'Close',
      made_by: 'Made by',
      original_by: 'original script by',
    },
    ru: {
      status_inactive: 'НЕАКТИВЕН',
      status_active: 'АКТИВЕН',
      hint_active: 'Нажмите триггер для суперглайда',
      hint_inactive: 'Нажмите для активации',
      btn_start: 'СТАРТ',
      btn_stop: 'СТОП',
      section_fps: 'ЧАСТОТА КАДРОВ',
      section_timing: 'ТАЙМИНГИ',
      section_keys: 'КЛАВИШИ',
      label_delay: 'ЗАДЕРЖКА',
      label_jump_hold: 'УДЕРЖАНИЕ JUMP',
      label_hold: 'УДЕРЖАНИЕ',
      desc_delay: 'Между прыжком и присядом',
      desc_jump_hold: 'Длительность нажатия прыжка',
      desc_hold: 'Длительность удержания',
      key_trigger: 'ТРИГГЕР',
      key_jump: 'ПРЫЖОК',
      key_crouch: 'ПРИСЕСТ',
      desc_trigger: 'Активирует макрос',
      desc_jump: 'Кнопка прыжка в игре',
      desc_crouch: 'Кнопка приседания в игре',
      press_any: 'НАЖМИТЕ…',
      capturing_hint: 'Нажмите любую клавишу…',
      triggered: 'TRIGGERED',
      tt_theme: 'Сменить тему',
      tt_minimize: 'Свернуть в трей',
      tt_close: 'Закрыть',
      made_by: 'Автор',
      original_by: 'оригинальный скрипт',
    },
  } as const;

  type Lang = keyof typeof TRANSLATIONS;

  let isDark = true;
  let isActive = false;
  let isCapturing: string | null = null;
  let flashTriggered = false;
  let saveTimeout: ReturnType<typeof setTimeout> | null = null;
  let isReady = false;

  let settings: Settings = {
    trigger_key: 'CapsLock',
    jump_key: 'Space',
    crouch_key: 'ControlLeft',
    fps: 180,
    delay_ms: 5.0,
    jump_hold_us: 500,
    crouch_hold_ms: 300,
    theme: 'dark',
    language: 'en',
  };

  const FPS_PRESETS = [
    { fps: 144, delay: 7.0 },
    { fps: 180, delay: 5.0 },
    { fps: 240, delay: 4.5 },
    { fps: 300, delay: 3.8 },
  ];

  const KEY_LABELS: Record<string, string> = {
    CapsLock: 'CAPS LOCK',
    Space: 'SPACE',
    ControlLeft: 'L-CTRL',
    ControlRight: 'R-CTRL',
    ShiftLeft: 'L-SHIFT',
    ShiftRight: 'R-SHIFT',
    Alt: 'ALT',
    AltGr: 'ALT GR',
    Return: 'ENTER',
    Escape: 'ESC',
    Tab: 'TAB',
    Backspace: 'BACKSPACE',
    UpArrow: '↑',
    DownArrow: '↓',
    LeftArrow: '←',
    RightArrow: '→',
    F1: 'F1', F2: 'F2', F3: 'F3', F4: 'F4',
    F5: 'F5', F6: 'F6', F7: 'F7', F8: 'F8',
    F9: 'F9', F10: 'F10', F11: 'F11', F12: 'F12',
    Num1: '1', Num2: '2', Num3: '3', Num4: '4', Num5: '5',
    Num6: '6', Num7: '7', Num8: '8', Num9: '9', Num0: '0',
    KeyA: 'A', KeyB: 'B', KeyC: 'C', KeyD: 'D', KeyE: 'E',
    KeyF: 'F', KeyG: 'G', KeyH: 'H', KeyI: 'I', KeyJ: 'J',
    KeyK: 'K', KeyL: 'L', KeyM: 'M', KeyN: 'N', KeyO: 'O',
    KeyP: 'P', KeyQ: 'Q', KeyR: 'R', KeyS: 'S', KeyT: 'T',
    KeyU: 'U', KeyV: 'V', KeyW: 'W', KeyX: 'X', KeyY: 'Y',
    KeyZ: 'Z',
    MetaLeft: 'WIN', MetaRight: 'WIN',
    Insert: 'INSERT', Delete: 'DELETE',
    Home: 'HOME', End: 'END',
    PageUp: 'PAGE UP', PageDown: 'PAGE DOWN',
  };

  // ─── Reactive derived state ───
  $: lang = (settings.language || 'en') as Lang;
  $: t = TRANSLATIONS[lang] ?? TRANSLATIONS.en;
  $: keyBinds = [
    { field: 'trigger_key' as const, label: t.key_trigger, desc: t.desc_trigger },
    { field: 'jump_key'    as const, label: t.key_jump,    desc: t.desc_jump    },
    { field: 'crouch_key'  as const, label: t.key_crouch,  desc: t.desc_crouch  },
  ];

  function displayKey(key: string): string {
    return KEY_LABELS[key] || key;
  }

  function selectFps(preset: { fps: number; delay: number }) {
    settings.fps = preset.fps;
    settings.delay_ms = preset.delay;
    settings = { ...settings };
    debounceSave();
  }

  async function toggleActive() {
    if (isActive) {
      await invoke('stop_macro');
      isActive = false;
    } else {
      await invoke('start_macro');
      isActive = true;
    }
  }

  const CODE_TO_RDEV: Record<string, string> = {

    ShiftLeft: 'ShiftLeft',   ShiftRight: 'ShiftRight',
    ControlLeft: 'ControlLeft', ControlRight: 'ControlRight',
    AltLeft: 'Alt',           AltRight: 'AltGr',
    MetaLeft: 'MetaLeft',     MetaRight: 'MetaRight',
    CapsLock: 'CapsLock',

    Space: 'Space',
    Enter: 'Return',
    NumpadEnter: 'KpReturn',
    Backspace: 'Backspace',
    Tab: 'Tab',
    Escape: 'Escape',
    Delete: 'Delete',
    Insert: 'Insert',

    Home: 'Home',             End: 'End',
    PageUp: 'PageUp',         PageDown: 'PageDown',
    ArrowUp: 'UpArrow',       ArrowDown: 'DownArrow',
    ArrowLeft: 'LeftArrow',   ArrowRight: 'RightArrow',

    F1: 'F1',   F2: 'F2',   F3: 'F3',   F4: 'F4',
    F5: 'F5',   F6: 'F6',   F7: 'F7',   F8: 'F8',
    F9: 'F9',   F10: 'F10', F11: 'F11', F12: 'F12',

    Digit1: 'Num1', Digit2: 'Num2', Digit3: 'Num3',
    Digit4: 'Num4', Digit5: 'Num5', Digit6: 'Num6',
    Digit7: 'Num7', Digit8: 'Num8', Digit9: 'Num9',
    Digit0: 'Num0',

    KeyA: 'KeyA', KeyB: 'KeyB', KeyC: 'KeyC', KeyD: 'KeyD',
    KeyE: 'KeyE', KeyF: 'KeyF', KeyG: 'KeyG', KeyH: 'KeyH',
    KeyI: 'KeyI', KeyJ: 'KeyJ', KeyK: 'KeyK', KeyL: 'KeyL',
    KeyM: 'KeyM', KeyN: 'KeyN', KeyO: 'KeyO', KeyP: 'KeyP',
    KeyQ: 'KeyQ', KeyR: 'KeyR', KeyS: 'KeyS', KeyT: 'KeyT',
    KeyU: 'KeyU', KeyV: 'KeyV', KeyW: 'KeyW', KeyX: 'KeyX',
    KeyY: 'KeyY', KeyZ: 'KeyZ',

    Backquote: 'BackQuote', Minus: 'Minus',       Equal: 'Equal',
    BracketLeft: 'LeftBracket', BracketRight: 'RightBracket',
    Backslash: 'BackSlash', Semicolon: 'SemiColon',
    Quote: 'Quote',  Comma: 'Comma', Period: 'Dot', Slash: 'Slash',

    Numpad0: 'Kp0', Numpad1: 'Kp1', Numpad2: 'Kp2', Numpad3: 'Kp3',
    Numpad4: 'Kp4', Numpad5: 'Kp5', Numpad6: 'Kp6', Numpad7: 'Kp7',
    Numpad8: 'Kp8', Numpad9: 'Kp9',
    NumpadAdd: 'KpPlus',     NumpadSubtract: 'KpMinus',
    NumpadMultiply: 'KpMultiply', NumpadDivide: 'KpDivide',
    NumpadDecimal: 'KpDelete',
    NumLock: 'NumLock',      ScrollLock: 'ScrollLock',
    Pause: 'Pause',          PrintScreen: 'PrintScreen',
  };

  function captureKey(field: string) {
    if (isCapturing === field) {
      isCapturing = null;
      return;
    }
    isCapturing = field;
    (document.activeElement as HTMLElement | null)?.blur();
  }

  function debounceSave() {
    if (saveTimeout) clearTimeout(saveTimeout);
    saveTimeout = setTimeout(async () => {
      try { await invoke('save_settings', { settings }); }
      catch (e) { console.error('Save failed:', e); }
    }, 500);
  }

  async function loadSettings() {
    try {
      const loaded = await invoke<Settings>('load_settings');
      settings = loaded;
      isDark = loaded.theme === 'dark';
    } catch { /* use defaults */ }
  }

  function toggleTheme() {
    isDark = !isDark;
    settings = { ...settings, theme: isDark ? 'dark' : 'light' };
    debounceSave();
  }

  function toggleLanguage() {
    settings = { ...settings, language: settings.language === 'en' ? 'ru' : 'en' };
    debounceSave();
  }

  async function openCreditsLink() {
    try { await invoke('open_url', { url: 'https://github.com/blaqarm' }); }
    catch (e) { console.error('open_url failed:', e); }
  }

  // ─── Window & lifecycle ───
  let unlistenCapture: (() => void) | undefined;
  let unlistenTriggered: (() => void) | undefined;
  let appWindow: Awaited<ReturnType<typeof getCurrentWindow>> | null = null;

  function handleWindowKeydown(e: KeyboardEvent) {
    if (!isCapturing) return;

    e.preventDefault();
    e.stopImmediatePropagation();

    if (e.code === 'Escape') {
      isCapturing = null;
      return;
    }

    const rdevKey = CODE_TO_RDEV[e.code];
    if (!rdevKey) return; // unknown key — wait for something recognisable

    settings = { ...settings, [isCapturing]: rdevKey };
    isCapturing = null;
    debounceSave();
  }

  function handleWindowKeyup(e: KeyboardEvent) {
    if (isCapturing) {
      e.preventDefault();
      e.stopImmediatePropagation();
    }
  }

  onMount(async () => {
    appWindow = getCurrentWindow();
    await loadSettings();

    window.addEventListener('keydown', handleWindowKeydown, true);
    window.addEventListener('keyup', handleWindowKeyup, true);

    unlistenCapture = await listen<{ field: string; key: string }>('key-captured', (event) => {
      const { field, key } = event.payload;
      settings = { ...settings, [field]: key };
      isCapturing = null;
      debounceSave();
    });

    unlistenTriggered = await listen('superglide-triggered', () => {
      flashTriggered = true;
      setTimeout(() => { flashTriggered = false; }, 300);
    });

    isReady = true;
    try { await appWindow.show(); } catch { /* dev mode — window already visible */ }
  });

  onDestroy(() => {
    window.removeEventListener('keydown', handleWindowKeydown, true);
    window.removeEventListener('keyup', handleWindowKeyup, true);
    unlistenCapture?.();
    unlistenTriggered?.();
    if (saveTimeout) clearTimeout(saveTimeout);
  });

  async function minimizeWindow() {
    try { await invoke('hide_window'); }
    catch (e) { console.error('hide:', e); }
  }

  async function closeWindow() {
    try { await appWindow?.close(); }
    catch (e) { console.error('close:', e); }
  }
</script>

<div class="app" class:dark={isDark} class:light={!isDark} class:ready={isReady}>

  <!-- ─── Title bar ─── -->
  <div class="titlebar" data-tauri-drag-region>
    <div class="titlebar-brand" data-tauri-drag-region>
      <span class="titlebar-icon">◈</span>
      <span class="titlebar-title" data-tauri-drag-region>
        SUPERGLIDE<span class="titlebar-dot">.RS</span>
      </span>
    </div>
    <div class="titlebar-actions">
      <button class="wm-btn minimize" on:click={minimizeWindow} title={t.tt_minimize}>
        <svg width="10" height="2" viewBox="0 0 10 2" fill="none">
          <rect width="10" height="2" rx="1" fill="currentColor"/>
        </svg>
      </button>
      <button class="wm-btn close" on:click={closeWindow} title={t.tt_close}>
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
          <path d="M1 1L9 9M9 1L1 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      </button>
    </div>
  </div>

  <div class="content">

    <!-- ─── Status ─── -->
    <div class="status-card" class:active={isActive} class:flash={flashTriggered}>
      <div class="status-left">
        <div class="status-indicator" class:active={isActive}>
          <span class="status-dot"></span>
        </div>
        <div class="status-text-group">
          <span class="status-label">{isActive ? t.status_active : t.status_inactive}</span>
          {#if flashTriggered}
            <span class="triggered-tag">{t.triggered}</span>
          {:else}
            <span class="status-hint">{isActive ? t.hint_active : t.hint_inactive}</span>
          {/if}
        </div>
      </div>
      <button class="activate-btn" class:active={isActive} on:click={toggleActive}>
        {isActive ? t.btn_stop : t.btn_start}
      </button>
    </div>

    <!-- ─── FPS ─── -->
    <div class="section">
      <div class="section-header">
        <span class="section-title">{t.section_fps}</span>
        <span class="section-badge">{settings.fps} FPS</span>
      </div>
      <div class="fps-row">
        {#each FPS_PRESETS as preset}
          <button
                  class="fps-btn"
                  class:selected={settings.fps === preset.fps}
                  on:click={() => selectFps(preset)}
          >{preset.fps}</button>
        {/each}
      </div>
    </div>

    <!-- ─── Timing ─── -->
    <div class="section">
      <div class="section-header">
        <span class="section-title">{t.section_timing}</span>
      </div>
      <div class="timing-grid">
        <div class="timing-item">
          <span class="timing-label">{t.label_delay}</span>
          <span class="timing-desc">{t.desc_delay}</span>
          <div class="timing-input-row">
            <input type="number" class="timing-input"
                   bind:value={settings.delay_ms} on:change={debounceSave}
                   min="0.1" max="50" step="0.1" />
            <span class="timing-unit">ms</span>
          </div>
        </div>
        <div class="timing-divider"></div>
        <div class="timing-item">
          <span class="timing-label">{t.label_jump_hold}</span>
          <span class="timing-desc">{t.desc_jump_hold}</span>
          <div class="timing-input-row">
            <input type="number" class="timing-input"
                   bind:value={settings.jump_hold_us} on:change={debounceSave}
                   min="100" max="5000" step="100" />
            <span class="timing-unit">µs</span>
          </div>
        </div>
        <div class="timing-divider"></div>
        <div class="timing-item">
          <span class="timing-label">{t.label_hold}</span>
          <span class="timing-desc">{t.desc_hold}</span>
          <div class="timing-input-row">
            <input type="number" class="timing-input"
                   bind:value={settings.crouch_hold_ms} on:change={debounceSave}
                   min="50" max="1000" step="10" />
            <span class="timing-unit">ms</span>
          </div>
        </div>
      </div>
    </div>

    <!-- ─── Key bindings ─── -->
    <div class="section">
      <div class="section-header">
        <span class="section-title">{t.section_keys}</span>
        {#if isCapturing}
          <span class="capturing-hint">{t.capturing_hint}</span>
        {/if}
      </div>
      <div class="keybinds">
        {#each keyBinds as bind}
          <div class="keybind-row">
            <div class="keybind-info">
              <span class="keybind-label">{bind.label}</span>
              <span class="keybind-desc">{bind.desc}</span>
            </div>
            <button
                    class="key-btn"
                    class:capturing={isCapturing === bind.field}
                    on:click={() => captureKey(bind.field)}
            >
              {#if isCapturing === bind.field}
                <span class="key-pulse">●</span> {t.press_any}
              {:else}
                {displayKey(settings[bind.field])}
              {/if}
            </button>
          </div>
        {/each}
      </div>
    </div>

  </div>

  <!-- ─── Footer ─── -->
  <div class="footer">
    <div class="footer-controls">
      <button class="icon-btn" on:click={toggleTheme} title={t.tt_theme}>
        {#if isDark}
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="5"/>
            <line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/>
            <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
            <line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/>
            <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
          </svg>
        {:else}
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
          </svg>
        {/if}
      </button>
      <button class="icon-btn lang-btn" on:click={toggleLanguage}
              title={settings.language === 'en' ? 'Переключить язык' : 'Switch language'}>
        {settings.language.toUpperCase()}
      </button>
    </div>

    <div class="footer-credits">
      <span class="credits-author">{t.made_by} <strong>Dmitry Osin</strong></span>
      <span class="credits-sep">·</span>
      <span class="credits-original">{t.original_by}</span>
      <button class="credits-link" on:click={openCreditsLink}>blaQarm</button>
    </div>
  </div>

</div>

<style>
  /* ─── Reset & Base ─── */
  :global(*, *::before, *::after) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :global(html), :global(body) {
    background: transparent;
    width: 100%;
    height: 100%;
    overflow: hidden;
    -webkit-font-smoothing: antialiased;
  }

  :global(#svelte) {
    width: 100%;
    height: 100%;
    background: transparent;
  }

  /* ─── Theme variables ─── */
  .app.dark {
    --bg: #0c0c0c;
    --surface: #141414;
    --surface-2: #1a1a1a;
    --surface-3: #222222;
    --border: #282828;
    --border-subtle: #1e1e1e;
    --text: #f2f2f2;
    --text-2: #888888;
    --text-3: #555555;
    --accent: #e63946;
    --accent-dim: rgba(230, 57, 70, 0.12);
    --accent-glow: rgba(230, 57, 70, 0.25);
    --active-bg: rgba(230, 57, 70, 0.08);
    --active-border: rgba(230, 57, 70, 0.4);
    --shadow: 0 0 0 1px rgba(255,255,255,0.04), 0 24px 64px rgba(0,0,0,0.8);
    --btn-bg: #1e1e1e;
    --btn-hover: #2a2a2a;
    --input-bg: #1a1a1a;
  }

  .app.light {
    --bg: #f0f0f0;
    --surface: #ffffff;
    --surface-2: #f8f8f8;
    --surface-3: #f0f0f0;
    --border: #e4e4e4;
    --border-subtle: #eeeeee;
    --text: #111111;
    --text-2: #777777;
    --text-3: #aaaaaa;
    --accent: #e63946;
    --accent-dim: rgba(230, 57, 70, 0.08);
    --accent-glow: rgba(230, 57, 70, 0.2);
    --active-bg: rgba(230, 57, 70, 0.06);
    --active-border: rgba(230, 57, 70, 0.35);
    --shadow: 0 0 0 1px rgba(0,0,0,0.06), 0 20px 60px rgba(0,0,0,0.2);
    --btn-bg: #f0f0f0;
    --btn-hover: #e8e8e8;
    --input-bg: #f5f5f5;
  }

  /* ─── App shell ─── */
  .app {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: var(--bg);
    border-radius: 12px;
    overflow: hidden;
    box-shadow: var(--shadow);
    border: 1px solid var(--border);
    font-family: 'Segoe UI', system-ui, -apple-system, sans-serif;
    color: var(--text);
    transition: background 0.2s ease, color 0.2s ease, opacity 0.18s ease;
    user-select: none;
    opacity: 0;
  }
  .app.ready {
    opacity: 1;
  }

  /* ─── Title bar ─── */
  .titlebar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px 0 14px;
    height: 42px;
    min-height: 42px;
    background: var(--surface);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .titlebar-brand {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: default;
  }

  .titlebar-icon {
    color: var(--accent);
    font-size: 14px;
    line-height: 1;
  }

  .titlebar-title {
    font-size: 12px;
    font-weight: 700;
    letter-spacing: 0.12em;
    color: var(--text);
  }

  .titlebar-dot {
    color: var(--accent);
  }

  .titlebar-actions {
    display: flex;
    gap: 4px;
  }

  .wm-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: none;
    background: transparent;
    color: var(--text-2);
    border-radius: 6px;
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
  }

  .wm-btn:hover { background: var(--btn-hover); color: var(--text); }
  .wm-btn.close:hover { background: var(--accent); color: #fff; }

  /* ─── Main content ─── */
  .content {
    flex: 1;
    overflow-y: auto;
    padding: 14px 14px 8px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    scrollbar-width: thin;
    scrollbar-color: var(--border) transparent;
  }

  .content::-webkit-scrollbar { width: 4px; }
  .content::-webkit-scrollbar-track { background: transparent; }
  .content::-webkit-scrollbar-thumb { background: var(--border); border-radius: 2px; }

  /* ─── Status card ─── */
  .status-card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 14px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 10px;
    transition: background 0.2s, border-color 0.2s, box-shadow 0.2s;
  }

  .status-card.active {
    background: var(--active-bg);
    border-color: var(--active-border);
    box-shadow: 0 0 20px var(--accent-glow);
  }

  .status-card.flash { animation: flash-pulse 0.3s ease; }

  @keyframes flash-pulse {
    0%  { box-shadow: 0 0 20px var(--accent-glow); }
    50% { box-shadow: 0 0 40px rgba(230, 57, 70, 0.6); }
    100% { box-shadow: 0 0 20px var(--accent-glow); }
  }

  .status-left { display: flex; align-items: center; gap: 10px; }

  .status-indicator {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: var(--surface-3);
    border: 1px solid var(--border);
    transition: background 0.2s, border-color 0.2s;
    flex-shrink: 0;
  }

  .status-indicator.active {
    background: var(--accent-dim);
    border-color: var(--accent);
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--text-3);
    transition: background 0.2s, box-shadow 0.2s;
  }

  .status-indicator.active .status-dot {
    background: var(--accent);
    box-shadow: 0 0 8px var(--accent);
    animation: dot-pulse 2s ease infinite;
  }

  @keyframes dot-pulse {
    0%, 100% { box-shadow: 0 0 6px var(--accent); }
    50%       { box-shadow: 0 0 14px var(--accent); }
  }

  .status-text-group { display: flex; flex-direction: column; gap: 2px; }

  .status-label {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.1em;
    color: var(--text);
  }

  .status-hint {
    font-size: 10px;
    color: var(--text-3);
    letter-spacing: 0.02em;
  }

  .triggered-tag {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.12em;
    color: var(--accent);
    background: var(--accent-dim);
    padding: 1px 5px;
    border-radius: 3px;
    border: 1px solid var(--accent);
    width: fit-content;
    animation: fade-in 0.1s ease;
  }

  @keyframes fade-in {
    from { opacity: 0; transform: translateY(-2px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  .activate-btn {
    padding: 8px 20px;
    border: 1px solid var(--border);
    border-radius: 7px;
    background: var(--btn-bg);
    color: var(--text-2);
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.1em;
    cursor: pointer;
    transition: all 0.15s ease;
    white-space: nowrap;
  }

  .activate-btn:hover,
  .activate-btn.active {
    background: var(--accent);
    border-color: var(--accent);
    color: #fff;
    box-shadow: 0 0 16px var(--accent-glow);
  }

  .activate-btn.active:hover {
    background: #cc2f3a;
    border-color: #cc2f3a;
  }

  /* ─── Sections ─── */
  .section {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 12px 14px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .section-title {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.14em;
    color: var(--text-2);
  }

  .section-badge {
    font-size: 10px;
    font-weight: 600;
    color: var(--accent);
    background: var(--accent-dim);
    padding: 2px 7px;
    border-radius: 4px;
    border: 1px solid rgba(230, 57, 70, 0.2);
    letter-spacing: 0.05em;
  }

  /* ─── FPS buttons ─── */
  .fps-row { display: flex; gap: 6px; }

  .fps-btn {
    flex: 1;
    padding: 7px 4px;
    border: 1px solid var(--border);
    border-radius: 7px;
    background: var(--btn-bg);
    color: var(--text-2);
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.04em;
    cursor: pointer;
    transition: all 0.15s ease;
    text-align: center;
  }

  .fps-btn:hover { border-color: var(--accent); color: var(--accent); background: var(--accent-dim); }
  .fps-btn.selected { background: var(--accent); border-color: var(--accent); color: #fff; box-shadow: 0 0 10px var(--accent-glow); }

  /* ─── Timing ─── */
  .timing-grid { display: flex; align-items: center; gap: 10px; }
  .timing-item { flex: 1; display: flex; flex-direction: column; gap: 4px; }

  .timing-label {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.1em;
    color: var(--text-2);
  }

  .timing-desc {
    font-size: 9px;
    color: var(--text-3);
    letter-spacing: 0.02em;
    line-height: 1.3;
  }

  .timing-input-row { display: flex; align-items: center; gap: 6px; margin-top: 2px; }

  .timing-input {
    width: 70px;
    padding: 5px 8px;
    background: var(--input-bg);
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text);
    font-size: 13px;
    font-weight: 500;
    font-family: 'Segoe UI Mono', 'Consolas', monospace;
    transition: border-color 0.15s;
    -moz-appearance: textfield;
    appearance: textfield;
  }

  .timing-input::-webkit-outer-spin-button,
  .timing-input::-webkit-inner-spin-button { -webkit-appearance: none; }
  .timing-input:focus { outline: none; border-color: var(--accent); }

  .timing-unit { font-size: 10px; color: var(--text-3); font-weight: 600; letter-spacing: 0.05em; }
  .timing-divider { width: 1px; height: 48px; background: var(--border); flex-shrink: 0; }

  /* ─── Key bindings ─── */
  .capturing-hint {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    color: var(--accent);
    animation: blink 1s ease infinite;
  }

  @keyframes blink {
    0%, 100% { opacity: 1; }
    50%       { opacity: 0.4; }
  }

  .keybinds { display: flex; flex-direction: column; gap: 6px; }

  .keybind-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 0;
    border-bottom: 1px solid var(--border-subtle);
  }

  .keybind-row:last-child { border-bottom: none; padding-bottom: 0; }
  .keybind-info { display: flex; flex-direction: column; gap: 2px; }

  .keybind-label {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.08em;
    color: var(--text);
  }

  .keybind-desc { font-size: 9px; color: var(--text-3); letter-spacing: 0.02em; }

  .key-btn {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 5px 12px;
    min-width: 100px;
    background: var(--surface-3);
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text);
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.08em;
    font-family: 'Segoe UI Mono', 'Consolas', monospace;
    cursor: pointer;
    transition: all 0.15s ease;
    text-align: center;
    justify-content: center;
    white-space: nowrap;
  }

  .key-btn:hover { border-color: var(--accent); color: var(--accent); background: var(--accent-dim); }

  .key-btn.capturing {
    border-color: var(--accent);
    background: var(--accent-dim);
    color: var(--accent);
    animation: border-pulse 1s ease infinite;
  }

  @keyframes border-pulse {
    0%, 100% { box-shadow: 0 0 0 0 var(--accent-glow); }
    50%       { box-shadow: 0 0 0 3px var(--accent-glow); }
  }

  .key-pulse { font-size: 8px; animation: dot-pulse 0.8s ease infinite; }

  /* ─── Footer ─── */
  .footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 7px 14px 9px;
    border-top: 1px solid var(--border);
    background: var(--surface);
    flex-shrink: 0;
    gap: 8px;
  }

  .footer-controls { display: flex; align-items: center; gap: 4px; flex-shrink: 0; }

  .icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: 1px solid var(--border);
    border-radius: 7px;
    background: var(--btn-bg);
    color: var(--text-2);
    cursor: pointer;
    transition: all 0.15s ease;
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.05em;
  }

  .icon-btn:hover { border-color: var(--accent); color: var(--accent); background: var(--accent-dim); }

  .lang-btn { min-width: 28px; padding: 0 4px; width: auto; }

  .footer-credits {
    display: flex;
    align-items: center;
    gap: 5px;
    flex-wrap: wrap;
    justify-content: flex-end;
  }

  .credits-author {
    font-size: 9px;
    color: var(--text-3);
    letter-spacing: 0.04em;
  }

  .credits-author strong {
    color: var(--text-2);
    font-weight: 600;
  }

  .credits-sep { font-size: 9px; color: var(--text-3); }

  .credits-original {
    font-size: 9px;
    color: var(--text-3);
    letter-spacing: 0.02em;
  }

  .credits-link {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.06em;
    color: var(--accent);
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
    text-decoration: underline;
    text-underline-offset: 2px;
    transition: opacity 0.15s;
  }

  .credits-link:hover { opacity: 0.75; }
</style>