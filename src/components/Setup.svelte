<script lang="ts">
    import { setApiToken } from "$lib/lifx";
    import { load } from "@tauri-apps/plugin-store";

    let token = $state("");
    let loading = $state(false);
    let error = $state("");

    let { onTokenSet } = $props<{ onTokenSet: () => void }>();

    async function handleSubmit() {
        loading = true;
        error = "";
        try {
            // Set token in backend (saves to keyring)
            await setApiToken(token);

            // Notify parent
            onTokenSet();
        } catch (e) {
            error =
                "Failed to set token. Please check your internet connection and try again.";
            console.error(e);
        } finally {
            loading = false;
        }
    }
</script>

<div class="flex flex-col items-center justify-center h-full p-6">
    <div
        class="w-full max-w-md bg-glass border border-border-glass rounded-3xl backdrop-blur-xl p-8 md:p-10 shadow-2xl relative overflow-hidden"
    >
        <!-- Subtle internal glow -->
        <div
            class="absolute top-0 left-1/2 -translate-x-1/2 w-32 h-1 bg-glass-active blur-xl"
        ></div>

        <div class="text-center mb-8">
            <div
                class="inline-flex items-center justify-center w-12 h-12 rounded-full bg-glass border border-border-glass mb-4 text-white"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="20"
                    height="20"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    ><rect
                        width="18"
                        height="11"
                        x="3"
                        y="11"
                        rx="2"
                        ry="2"
                    /><path d="M7 11V7a5 5 0 0 1 10 0v4" /></svg
                >
            </div>
            <h1 class="text-2xl font-bold text-white mb-2">
                Welcome to Oculus
            </h1>
            <p class="text-text-secondary text-sm">
                Enter your LIFX API token to begin.
            </p>
        </div>

        <form
            onsubmit={(e) => {
                e.preventDefault();
                handleSubmit();
            }}
            class="space-y-4"
        >
            <div class="space-y-1.5">
                <label
                    for="token"
                    class="text-xs font-medium text-text-tertiary ml-1 uppercase tracking-wider"
                    >API Token</label
                >
                <input
                    id="token"
                    type="password"
                    bind:value={token}
                    placeholder="c76a..."
                    class="w-full bg-glass-input border border-border-glass rounded-xl px-4 py-3.5 text-white placeholder-text-active focus:outline-none focus:border-border-glass-focus focus:ring-1 focus:ring-border-border-glass-focus transition-all font-mono text-sm"
                    required
                />
            </div>

            {#if error}
                <div
                    class="p-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-200 text-xs text-center"
                >
                    {error}
                </div>
            {/if}

            <button
                type="submit"
                disabled={loading}
                class="w-full bg-white text-black font-bold rounded-xl py-3.5 hover:bg-gray-200 active:scale-[0.98] transition-all disabled:opacity-50 disabled:cursor-not-allowed mt-2 shadow-lg shadow-glass"
            >
                {loading ? "Connecting..." : "Connect"}
            </button>
        </form>

        <div class="mt-8 text-center">
            <a
                href="https://cloud.lifx.com/settings"
                target="_blank"
                rel="noopener noreferrer"
                class="text-xs text-text-muted hover:text-white transition-colors border-b border-transparent hover:border-text-muted pb-0.5"
            >
                Generate a token at cloud.lifx.com
            </a>
        </div>
    </div>
</div>
