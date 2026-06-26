<script lang="ts">
    import { t } from '$lib/i18n';
    import { getMeta } from '$lib';

    // The DB is already loaded by the time any route renders (the layout gates
    // children on it), so reading the corpus meta here is safe and synchronous.
    const meta = getMeta();
    const appLabel = `${__APP_VERSION__}${__APP_COMMIT__ ? ` (${__APP_COMMIT__})` : ''}`;
    // $derived so the localized "texts" label tracks the active UI language.
    const corpusParts = $derived(
        meta
            ? [
                  meta.data_commit,
                  meta.data_commit_date,
                  meta.book_count ? `${meta.book_count} ${t('pages.contribute.versionBooks')}` : '',
                  meta.languages
              ].filter(Boolean)
            : []
    );
</script>

<main id="main-content" tabindex="-1" class="max-w-4xl mx-auto px-4 py-12">
    <h1 class="text-2xl font-display font-bold text-foreground mb-6">{t('pages.contribute.title')}</h1>
    <div class="text-foreground leading-relaxed">
        {@html t('pages.contribute.body')}
    </div>

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
            {#if meta?.built_at}
                <div class="flex gap-2">
                    <dt class="font-medium text-foreground">{t('pages.contribute.versionBuilt')}</dt>
                    <dd>{meta.built_at}</dd>
                </div>
            {/if}
        </dl>
    </section>
</main>
