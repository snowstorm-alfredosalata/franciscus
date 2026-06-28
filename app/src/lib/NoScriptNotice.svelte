<script lang="ts">
	import { t } from '$lib/i18n';

	// Shown only when JavaScript is disabled (the browser renders <noscript>
	// content and ignores the live app). Lives in the prerendered HTML so no-JS
	// visitors and crawlers see why the dynamic parts are inert. The message is
	// our own controlled i18n string, so {@html} (for the <em>) is safe.
</script>

<noscript>
	<div
		role="alert"
		class="flex items-start gap-3 rounded-lg border border-destructive/50 bg-destructive/10
		       px-4 py-3 text-destructive"
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="mt-0.5 size-5 shrink-0"
			aria-hidden="true"
		>
			<path d="M10.29 3.86 1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0Z" />
			<line x1="12" y1="9" x2="12" y2="13" />
			<line x1="12" y1="17" x2="12.01" y2="17" />
		</svg>
		<p class="text-sm leading-relaxed">{@html t('app.noScript')}</p>
	</div>
</noscript>
