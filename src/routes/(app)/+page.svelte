<script lang="ts">
  import { onMount } from 'svelte';
  import gsap from 'gsap';
  import { Plus, FolderOpen, Clock, Settings } from 'lucide-svelte';
  import { goto } from '$app/navigation';

  let root: HTMLDivElement;
  let isLoading = $state(false);

  onMount(() => {
    const tl = gsap.timeline({ defaults: { ease: 'power3.out' } });

    tl.from(root, { opacity: 0, y: 20, scale: 0.97, duration: 0.7 })
      .from('.line', { y: 30, opacity: 0, stagger: 0.12 }, '-=0.4')
      .from('.action', { y: 20, opacity: 0, stagger: 0.15 }, '-=0.3');
  });
  async function startNew() {
  // 1. Create an Exit Timeline
  const tl = gsap.timeline({
    onComplete: () => {
      // 2. ONLY navigate after animation finishes
      goto('/new'); 
    }
  });

  // 3. Define the Exit Animation
  // "Stagger elements upwards and fade them out"
  tl.to('.gsap-card', { 
    y: -40, 
    opacity: 0, 
    stagger: 0.1, 
    duration: 0.4, 
    ease: 'power2.in' 
  })
  .to('.gsap-hero-line', { 
    y: -30, 
    opacity: 0, 
    stagger: 0.05, 
    duration: 0.4 
  }, '-=0.3') // Overlap slightly
  .to('.gsap-header', { 
    y: -20, 
    opacity: 0, 
    duration: 0.3 
  }, '-=0.3');
}
async function openPage() {
  // 1. Create an Exit Timeline
  const tl = gsap.timeline({
    onComplete: () => {
      // 2. ONLY navigate after animation finishes
      goto('/open'); 
    }
  });

  // 3. Define the Exit Animation
  // "Stagger elements upwards and fade them out"
  tl.to('.gsap-card', { 
    y: -40, 
    opacity: 0, 
    stagger: 0.1, 
    duration: 0.4, 
    ease: 'power2.in' 
  })
  .to('.gsap-hero-line', { 
    y: -30, 
    opacity: 0, 
    stagger: 0.05, 
    duration: 0.4 
  }, '-=0.3') // Overlap slightly
  .to('.gsap-header', { 
    y: -20, 
    opacity: 0, 
    duration: 0.3 
  }, '-=0.3');
}

async function resumeLast() {
  isLoading = true;
  // Simulating a process (you can replace this with your actual logic)
  setTimeout(() => { isLoading = false; }, 3000);
}
</script>

<div bind:this={root} class="stage" class:blurred={isLoading}>
  <!-- HERO -->
  <section class="hero">
    <h1 class="line">Hello.</h1>
    <h1 class="line accent">I’m Protégé.</h1>
    <p class="line">
      I don’t accelerate your typing.  
      I accelerate your thinking.
    </p>
  </section>

  <!-- ACTIONS -->
  <section class="actions">
    <button onclick={startNew} class="action primary">
      <Plus size="22" />
      <span>Start new project</span>
    </button>

    <button class="action" onclick={openPage}>
      <FolderOpen size="22" />
      <span>Open workspace</span>
    </button>

    <button class="action subtle" onclick={resumeLast}>
      <Clock size="18" />
      <span>Resume last</span>
    </button>
  </section>

  <!-- SETTINGS -->
  <button class="settings">
    <Settings size="20" />
  </button>

  {#if isLoading}
    <div class="loading-overlay">
      <div class="spinner"></div>
    </div>
  {/if}
</div>

<style>
  /* ---------- GLOBAL FIXES ---------- */
  :global(html, body) {
    height: 100%;
    margin: 0;
  }

  :global(body) {
    background: transparent;
    color-scheme: dark;
  }

  /* ---------- LAYOUT ---------- */
  .stage {
    height: calc(100vh - 42px); /* below your existing topbar */
    width: 100vw;

    box-sizing: border-box;
    padding: 64px 72px;

    display: flex;
    flex-direction: column;
    justify-content: space-around;

    position: relative;
    background: transparent; /* let Mica show */
    transition: filter 0.3s ease;
  }

  .stage.blurred {
    filter: blur(8px);
    pointer-events: none;
  }

  /* ---------- LOADING OVERLAY ---------- */
  .loading-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: calc(100vh - 42px);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
    background: rgba(0, 0, 0, 0.2);
  }

  .spinner {
    width: 50px;
    height: 50px;
    border: 3px solid rgba(147, 197, 253, 0.3);
    border-radius: 50%;
    border-top-color: #93c5fd;
    animation: spin 1s ease-in-out infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  /* ---------- HERO ---------- */
  .hero h1 {
    margin: 0;
    font-size: 3rem;
    font-weight: 600;
    letter-spacing: -0.02em;
    color: #f9fafb;
  }

  .hero p {
    margin-top: 18px;
    max-width: 520px;
    line-height: 1.5;
    font-size: 1rem;
    color: #d1d5db;
  }

  .accent {
    background: linear-gradient(
      90deg,
      #93c5fd,
      #c4b5fd,
      #67e8f9
    );
    -webkit-background-clip: text;
    background-clip: text;
    color: transparent;
  }

  /* ---------- ACTIONS ---------- */
  .actions {
    display: flex;
    gap: 22px;
    flex-wrap: wrap;
    align-items: center;
  }

  .action {
    position: relative;
    display: flex;
    align-items: center;
    gap: 12px;

    padding: 16px 22px;
    border-radius: 14px;

    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.18);
    backdrop-filter: blur(12px);

    cursor: pointer;

    transition:
      transform 0.25s ease,
      box-shadow 0.25s ease,
      background 0.25s ease;
  }

  /* LIGHT ICONS — HIGH CONTRAST */
  .action svg {
    color: #f9fafb; /* light icons on dark mica */
  }

  .action span {
    font-size: 0.95rem;
    letter-spacing: 0.2px;
    color: #f3f4f6;
  }

  /* GLOW LAYER */
  .action::after {
    content: '';
    position: absolute;
    inset: 0;
    border-radius: inherit;
    background: radial-gradient(
      500px circle at 50% 50%,
      rgba(147, 197, 253, 0.25),
      transparent 45%
    );
    opacity: 0;
    transition: opacity 0.25s ease;
  }

  .action:hover::after {
    opacity: 1;
  }

  .action:hover {
    transform: translateY(-4px) scale(1.02);
    box-shadow: 0 18px 36px rgba(0,0,0,0.45);
    background: rgba(255,255,255,0.12);
  }

  .action.primary {
    background: linear-gradient(
      135deg,
      rgba(147, 197, 253, 0.25),
      rgba(147, 197, 253, 0.08)
    );
    border-color: rgba(147, 197, 253, 0.45);
  }

  .action.subtle {
    opacity: 0.85;
  }

  /* ---------- SETTINGS ---------- */
  .settings {
    position: absolute;
    bottom: 22px;
    right: 22px;

    width: 44px;
    height: 44px;
    border-radius: 50%;

    background: rgba(255,255,255,0.1);
    border: 1px solid rgba(255,255,255,0.2);
    backdrop-filter: blur(10px);

    display: grid;
    place-items: center;
    cursor: pointer;

    transition:
      transform 0.4s ease,
      background 0.3s ease;
  }

  .settings svg {
    color: #f9fafb; /* light icon */
  }

  .settings:hover {
    transform: rotate(30deg) scale(1.1);
    background: rgba(255,255,255,0.18);
  }

  /* ---------- RESPONSIVE ---------- */
  @media (max-width: 720px) {
    .stage {
      padding: 36px 32px;
    }

    .hero h1 {
      font-size: 2.3rem;
    }
  }
</style>
