<script lang="ts">
    import { t } from '$lib/i18n';
    import type { PageData } from './$types';

    // Corpus provenance comes from the manifest (root layout load), so this hub
    // renders and prerenders without the sql.js DB.
    let { data }: { data: PageData } = $props();
    const corpus = $derived(data.manifest.corpus);
    const appLabel = `${__APP_VERSION__}${__APP_COMMIT__ ? ` (${__APP_COMMIT__})` : ''}`;
    // $derived so the localized "texts" label tracks the active UI language.
    const corpusParts = $derived(
        [
            corpus.data_commit,
            corpus.data_commit_date,
            corpus.book_count ? `${corpus.book_count} ${t('pages.contribute.versionBooks')}` : '',
            corpus.languages.join(', ')
        ].filter(Boolean)
    );
</script>

<main id="main-content" tabindex="-1" class="max-w-4xl mx-auto px-4 py-12">
    <h1 class="text-2xl font-display font-bold text-foreground mb-6">{t('pages.contribute.title')}</h1>
    <div class="text-foreground leading-relaxed">
        {@html t('pages.contribute.body')}
    </div>

    <section class="mt-8 border-t border-border pt-6">
        <h2 class="text-xs font-semibold uppercase tracking-[0.2em] text-muted-foreground/80 mb-3">
            {t('pages.contribute.connectTitle')}
        </h2>
        <div class="text-foreground leading-relaxed">
            {@html t('pages.contribute.connectBody')}
        </div>
        <a
            href="https://discord.gg/4QWwjRWbT"
            target="_blank"
            rel="noopener noreferrer"
            class="inline-flex items-center gap-2 rounded-md bg-[#5865F2] px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-[#4752C4] focus:outline-none focus:ring-2 focus:ring-[#5865F2] focus:ring-offset-2 focus:ring-offset-background"
        >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-5 h-5" aria-hidden="true">
                <path d="M20.317 4.3698a19.7913 19.7913 0 00-4.8851-1.5152.0741.0741 0 00-.0785.0371c-.211.3753-.4447.8648-.6083 1.2495-1.8447-.2762-3.68-.2762-5.4868 0-.1636-.3933-.4058-.8742-.6177-1.2495a.077.077 0 00-.0785-.037 19.7363 19.7363 0 00-4.8852 1.515.0699.0699 0 00-.0321.0277C.5334 9.0458-.319 13.5799.0992 18.0578a.0824.0824 0 00.0312.0561c2.0528 1.5076 4.0413 2.4228 5.9929 3.0294a.0777.0777 0 00.0842-.0276c.4616-.6304.8731-1.2952 1.226-1.9942a.076.076 0 00-.0416-.1057c-.6528-.2476-1.2743-.5495-1.8722-.8923a.077.077 0 01-.0076-.1277c.1258-.0943.2517-.1923.3718-.2914a.0743.0743 0 01.0776-.0105c3.9278 1.7933 8.18 1.7933 12.0614 0a.0739.0739 0 01.0785.0095c.1202.099.246.1981.3728.2924a.077.077 0 01-.0066.1276 12.2986 12.2986 0 01-1.873.8914.0766.0766 0 00-.0407.1067c.3604.698.7719 1.3628 1.225 1.9932a.076.076 0 00.0842.0286c1.961-.6067 3.9495-1.5219 6.0023-3.0294a.077.077 0 00.0313-.0552c.5004-5.177-.8382-9.6739-3.5485-13.6604a.061.061 0 00-.0312-.0286zM8.02 15.3312c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9554-2.4189 2.1569-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.9554 2.4189-2.1568 2.4189zm7.9748 0c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9554-2.4189 2.1569-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.946 2.4189-2.1568 2.4189Z"/>
            </svg>
            {t('pages.contribute.discord')}
        </a>
    </section>

    <section class="mt-10 border-t border-border pt-6 text-sm text-muted-foreground">
        <h2 class="text-xs font-semibold uppercase tracking-[0.2em] text-muted-foreground/80 mb-3">
            {t('pages.contribute.versionTitle')}
        </h2>
        <dl class="space-y-1 tabular-nums">
            <div class="flex gap-2">
                <dt class="font-medium text-foreground">{t('pages.contribute.versionApp')}</dt>
                <dd>{appLabel}</dd>
            </div>
            {#if corpusParts.length}
                <div class="flex gap-2">
                    <dt class="font-medium text-foreground">{t('pages.contribute.versionCorpus')}</dt>
                    <dd>{corpusParts.join(' · ')}</dd>
                </div>
            {/if}
            {#if corpus.built_at}
                <div class="flex gap-2">
                    <dt class="font-medium text-foreground">{t('pages.contribute.versionBuilt')}</dt>
                    <dd>{corpus.built_at}</dd>
                </div>
            {/if}
        </dl>
    </section>
</main>
