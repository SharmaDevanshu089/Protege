<script lang="ts">
  import { slide } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';
  import { X } from 'lucide-svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  
  // Tauri window handling
  let isDropdownOpen = false;

  function toggleDropdown() {
    isDropdownOpen = !isDropdownOpen;
  }

  function clickOutside(node: HTMLElement) {
    const handleClick = (event: MouseEvent) => {
      if (!node.contains(event.target as Node)) {
        isDropdownOpen = false;
      }
    };
    document.addEventListener('click', handleClick, true);
    return {
      destroy() {
        document.removeEventListener('click', handleClick, true);
      }
    };
  }

  async function closeApp() {
    console.log("Close button clicked");
    const window = getCurrentWindow();
    await window.close();
  }
</script>

<header class="titlebar" data-tauri-drag-region>
  
  <div class="project-menu" use:clickOutside>
    <button 
      class="menu-trigger" 
      class:active={isDropdownOpen} 
      on:click={toggleDropdown}
    >
      <span class="project-name">Protégé</span>
      
      <div class="icon-wrapper" class:rotated={isDropdownOpen}>
        <!-- Empty dropdown icon container - keeps layout consistent -->
      </div>
    </button>
  </div>

  <div class="drag-spacer" data-tauri-drag-region></div>

  <div class="window-controls">
    <button class="control-btn close-btn" on:click={closeApp} aria-label="Close">
      <X size={16} />
    </button>
  </div>

</header>

<style>
  :global(:root) {
    --tb-height: 42px; 
    --font-ui: "Segoe UI Variable", "Segoe UI", sans-serif;
    
    /* MICA VARS */
    --tb-hover: rgba(255, 255, 255, 0.06); 
    --tb-active: rgba(255, 255, 255, 0.1);
    --glass-bg: rgba(20, 20, 20, 0.25); 
    --glass-border: rgba(255, 255, 255, 0.08);
  }

  /* LAYOUT */
  .titlebar {
    height: var(--tb-height);
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 0 0 12px; 
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 9999;
    user-select: none;
    font-family: var(--font-ui);
    color: AccentColorText;
  }

  /* DRAG FIX */
  .drag-spacer {
    flex-grow: 1;
    height: 100%;
    pointer-events: auto; 
  }

  /* LEFT MENU */
  .project-menu {
    position: relative;
    margin-right: 8px;
  }

  .menu-trigger {
    display: flex;
    align-items: center;
    gap: 8px;
    background: transparent;
    border: none;
    color: #eeeeee;
    padding: 4px 8px;
    border-radius: 4px;
    cursor: pointer;
    transition: background 0.1s;
    height: 34px;
  }

  .project-name {
    font-size: 16px; 
    font-weight: 400; 
    letter-spacing: 0.3px;
  }

  .menu-trigger:hover {
    background: var(--tb-hover);
  }

  .menu-trigger.active {
    background: var(--tb-active);
  }

  .icon-wrapper {
    display: flex;
    transition: transform 0.2s cubic-bezier(0, 0, 0, 1);
    opacity: 0.8;
    width: 14px; /* Fixed width for consistent spacing */
    height: 14px;
  }

  .rotated {
    transform: rotate(180deg);
  }

  /* RIGHT CONTROLS */
  .window-controls {
    display: flex;
    height: 100%;
    -webkit-app-region: no-drag; 
  }

  .control-btn {
    width: 48px; 
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: #ffffff;
    cursor: default;
    transition: background 0.1s;
    border-radius: 0;
  }

  .control-btn:hover {
    background: rgba(255, 255, 255, 0.05);
  }

  .close-btn:hover {
    background: #c42b1c;
    color: white;
  }
</style>
