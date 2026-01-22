<script>
	import { onMount } from 'svelte';
	import { gsap } from 'gsap';

	let isLoaded = false;

	onMount(async () => {
		await import('deep-chat');
		isLoaded = true;

		// Title intro
		gsap.from('.title', {
			y: 80,
			opacity: 0,
			duration: 1,
			ease: 'power3.out'
		});

		// Chat pop-in
		gsap.from('.chat-wrapper', {
			scale: 0.96,
			opacity: 0,
			duration: 1,
			ease: 'power3.out',
			delay: 0.2
		});
	});
</script>

<main>
	<header>
		<h1 class="title">Let’s Create a Project</h1>
	</header>

	{#if isLoaded}
		<section class="chat-wrapper">
			<deep-chat
				connect={{ url: 'http://localhost:8000/chat' }}
				textInput={{ placeholder: { text: 'Ask, build, or explore…' } }}
			/>
		</section>
	{/if}
</main>

<style>
	/* ================= ROOT ================= */

	main {
		height: 100vh;
		width: 100%;
		display: flex;
		flex-direction: column;
		padding: 28px;
		box-sizing: border-box;

		background: transparent;
		color: #f9fafb;
		font-family: "Segoe UI", system-ui, sans-serif;
	}

	/* ================= HEADER ================= */

	header {
		text-align: center;
		margin-bottom: 20px;
	}

	.title {
		margin: 0;
		font-size: 2.6rem;
		font-weight: 600;
		letter-spacing: -0.03em;
	}

	/* ================= CHAT PANEL ================= */

	.chat-wrapper {
		flex: 1;
		min-height: 0;
		padding: 14px;

		background: rgba(18, 18, 20, 0.72);
		backdrop-filter: blur(18px);

		border-radius: 14px;
		box-shadow:
			0 10px 40px rgba(0, 0, 0, 0.45),
			inset 0 0 0 1px rgba(255, 255, 255, 0.06);
	}

	/* ================= DEEP CHAT THEME ================= */

	deep-chat {
		height: 100%;
		width: 100%;

		--chat-background: transparent;
		--font-family: "Segoe UI", system-ui, sans-serif;
		--text-color: #f9fafb;

		/* Messages */
		--message-user-background: linear-gradient(135deg, #2563eb, #1d4ed8);
		--message-user-text-color: #ffffff;

		--message-ai-background: rgba(39, 39, 42, 0.85);
		--message-ai-text-color: #e5e7eb;

		--message-border-radius: 12px;
		--message-padding: 12px 14px;

		/* Input */
		--input-background: rgba(24, 24, 27, 0.95);
		--input-border-color: rgba(255, 255, 255, 0.08);
		--input-border-radius: 10px;
		--input-text-color: #f9fafb;
		--input-placeholder-color: #9ca3af;

		/* Buttons */
		--submit-button-background: #2563eb;
		--submit-button-hover-background: #1d4ed8;
		--submit-button-border-radius: 8px;

		/* Scrollbar */
		--scrollbar-thumb-color: rgba(255, 255, 255, 0.15);
	}

	/* ================= MESSAGE ANIMATION ================= */

	deep-chat::part(message) {
		animation: msgIn 0.22s ease-out;
	}

	@keyframes msgIn {
		from {
			opacity: 0;
			transform: translateY(6px) scale(0.97);
		}
		to {
			opacity: 1;
			transform: none;
		}
	}
</style>
