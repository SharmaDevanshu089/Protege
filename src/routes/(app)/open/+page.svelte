<script lang="ts">
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';
  import { FolderOpen } from 'lucide-svelte';
  import { invoke } from "@tauri-apps/api/core";

  // State
  let isFolderSelected = false;

  // DOM Element References for GSAP
  let overlayRef: HTMLDivElement;
  let textRef: HTMLDivElement;
  let mainContentRef: HTMLDivElement;

  onMount(() => {
    // 1. ON LOAD ANIMATION
    // The text smoothly slides up and fades in
    gsap.from(textRef, {
      y: 40,
      opacity: 0,
      duration: 0.8,
      ease: "power3.out",
      delay: 0.2 // Slight delay so the user sees it happen
    });
  });

  function handleSelectFolder() {
    let selectedPath = invoke("select_vault_folder");
    console.log(selectedPath);
    // 2. TRIGGER FOLDER SELECTION
    // In your real app, you will call the Tauri dialog here:
    // const selectedPath = await open({ directory: true });
    
    // For now, we just run the exit animation
    
    // Animate the overlay fading away
    gsap.to(overlayRef, {
      opacity: 0,
      backdropFilter: "blur(0px)",
      duration: 0.4,
      ease: "power2.inOut",
      onComplete: () => {
        // Once the overlay is gone, update the state to re-enable the background
        isFolderSelected = true;
        
        // Animate the main content popping in slightly
        gsap.fromTo(mainContentRef, 
          { scale: 0.98, opacity: 0.8 },
          { scale: 1, opacity: 1, duration: 0.5, ease: "back.out(1.2)", clearProps: "all" }
        );
      }
    });
  }

  function animateDots(node: HTMLElement) {
    gsap.to(node.querySelectorAll('.dot'), {
      opacity: 0.2,
      stagger: 0.2,
      repeat: -1,
      yoyo: true,
      ease: "power1.inOut"
    });
  }
</script>

<div class="page-container">
  
  {#if !isFolderSelected}
    <div class="overlay" bind:this={overlayRef}>
      <div class="overlay-box" bind:this={textRef}>
        <h2>Please select the folder</h2>
        <p class="subtitle">Choose a directory to act as your secure vault.</p>
        
        <button class="primary-btn" on:click={handleSelectFolder}>
          <FolderOpen size={18} />
          <span>Select Folder</span>
        </button>
      </div>
    </div>
  {/if}

  <div 
    class="main-content" 
    class:disabled={!isFolderSelected} 
    bind:this={mainContentRef}
  >
    <div class="loading-container">
      <h1 
        class="loading-text"
        use:animateDots
      >
        Opening<span class="dot">.</span><span class="dot">.</span><span class="dot">.</span>
      </h1>
    </div>
        
  </div>

</div>

<style>
  .loading-container {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
  }

    :global(html, body) {
    height: 100%;
    margin: 0;
  }

  :global(body) {
    background: transparent;
    color-scheme: dark;
  }

  /* Base Variables */
  :root {
    --accent: #60cdff;
    --accent-hover: #7aceff;
    --text-primary: #ffffff;
    --text-secondary: #a0a0a0;
  }

  .page-container {
    position: relative;
    width: 100%;
    height: calc(100vh - 42px);
    overflow: hidden;
    color: var(--text-primary);
    font-family: "Segoe UI Variable", sans-serif;
  }

  /* --- OVERLAY --- */
  .overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    /* Dark semi-transparent background with a strong blur */
    background: rgba(0, 0, 0, 0.4);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    z-index: 50;
    
    /* Center the prompt box */
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .overlay-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    background: rgba(30, 30, 30, 0.6);
    border: 1px solid rgba(255, 255, 255, 0.1);
    padding: 40px;
    border-radius: 12px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  }

  .overlay-box h2 {
    margin: 0 0 8px 0;
    font-size: 24px;
    font-weight: 600;
  }

  .subtitle {
    margin: 0 0 24px 0;
    color: var(--text-secondary);
    font-size: 14px;
  }

  /* --- MAIN CONTENT --- */
  .main-content {
    padding: 40px;
    height: 100%;
    transition: all 0.3s ease;
  }

  /* The "Disabled" Look */
  .main-content.disabled {
    filter: blur(4px) grayscale(50%); /* Blurs and washes out the background */
    opacity: 0.4;
    pointer-events: none; /* Prevents any clicking behind the overlay */
    user-select: none;
  }

  /* --- WINDOWS 11 BUTTON --- */
  .primary-btn {
    background: var(--accent);
    color: #000;
    border: none;
    padding: 10px 24px;
    border-radius: 6px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 8px;
    transition: all 0.2s ease;
  }

  .primary-btn:hover {
    background: var(--accent-hover);
    transform: scale(1.02);
  }

  .primary-btn:active {
    transform: scale(0.98);
  }

  /* Dummy UI for visual effect */
  .dummy-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 20px;
    margin-top: 40px;
  }

  .dummy-card {
    height: 120px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
  }
</style>