<script lang="ts">
    import { setApiToken, getLights } from "$lib/lifx";
    import { load } from "@tauri-apps/plugin-store";
    import { fade } from "svelte/transition";
    import { goto } from "$app/navigation";

    let token = $state("");
    let loading = $state(false);
    let message = $state("");
    let messageType = $state<"success" | "error">("success");

    async function handleSave() {
        if (!token.trim()) {
            message = "Please enter a valid token";
            messageType = "error";
            return;
        }

        loading = true;
        message = "";

        try {
            // 1. Set token in backend state (which now saves to keyring)
            await setApiToken(token);

            // 2. Verify token works
            await getLights();

            message = "Token verified and saved successfully";
            token = ""; // Clear for security/cleanliness
        } catch (e) {
            console.error("Failed to set token:", e);
            message = "Invalid token. Please check and try again.";
            messageType = "error";

            // Optional: Revert backend state if needed, though next attempt will overwrite
        } finally {
            loading = false;
        }
    }

    let confirmingDisconnect = $state(false);

    async function handleDisconnect() {
        if (!confirmingDisconnect) {
            confirmingDisconnect = true;
            // Reset after 3 seconds if not confirmed
            setTimeout(() => {
                confirmingDisconnect = false;
            }, 3000);
            return;
        }

        try {
            console.log("Disconnecting...");
            // Just set empty token to clear it from keyring
            await setApiToken("");
            message = "Disconnected successfully";
            window.location.href = "/";
        } catch (e) {
            console.error("Failed to disconnect:", e);
            message = "Failed to disconnect. Check console.";
            messageType = "error";
            confirmingDisconnect = false;
        }
    }
</script>

<div
    class="min-h-screen w-full bg-surface text-white relative overflow-hidden flex flex-col"
>
    <!-- Background Elements -->
    <div
        class="absolute top-[-20%] left-[-10%] w-[50%] h-[50%] bg-purple-900/20 blur-[120px] rounded-full pointer-events-none"
    ></div>
    <div
        class="absolute bottom-[-20%] right-[-10%] w-[50%] h-[50%] bg-blue-900/20 blur-[120px] rounded-full pointer-events-none"
    ></div>

    <!-- Header -->
    <header class="p-6 md:p-8 flex items-center justify-between relative z-10">
        <div class="flex items-center gap-4">
            <a
                href="/"
                class="p-2.5 rounded-full bg-glass hover:bg-glass-hover border border-border-glass text-text-secondary hover:text-white transition-all active:scale-95 group"
                aria-label="Back to Dashboard"
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
                    class="group-hover:-translate-x-0.5 transition-transform"
                    ><path d="m15 18-6-6 6-6" /></svg
                >
            </a>
            <h1 class="text-2xl font-bold tracking-tight">Settings</h1>
        </div>
    </header>

    <!-- Content -->
    <main class="flex-1 w-full max-w-2xl mx-auto px-6 pb-12 relative z-10">
        <div class="space-y-8">
            <!-- API Token Section -->
            <section
                class="bg-glass border border-border-glass rounded-3xl p-8 backdrop-blur-xl relative overflow-hidden"
            >
                <div
                    class="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-transparent via-white/10 to-transparent"
                ></div>

                <div class="flex items-start gap-4 mb-6">
                    <div
                        class="p-3 rounded-xl bg-glass-active border border-border-glass text-white"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
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
                    <div>
                        <h2 class="text-lg font-bold text-white">
                            LIFX API Token
                        </h2>
                        <p class="text-text-secondary text-sm mt-1">
                            Update your personal access token to control your
                            lights.
                        </p>
                    </div>
                </div>

                <form
                    onsubmit={(e) => {
                        e.preventDefault();
                        handleSave();
                    }}
                    class="space-y-4"
                >
                    <div class="space-y-1.5">
                        <label
                            for="token"
                            class="text-xs font-medium text-text-tertiary ml-1 uppercase tracking-wider"
                            >New Token</label
                        >
                        <input
                            id="token"
                            type="password"
                            bind:value={token}
                            placeholder="Enter new token..."
                            class="w-full bg-glass-input border border-border-glass rounded-xl px-4 py-3.5 text-white placeholder-text-muted focus:outline-none focus:border-border-glass-focus focus:ring-1 focus:ring-border-border-glass-focus transition-all font-mono text-sm"
                        />
                    </div>

                    {#if message}
                        <div
                            in:fade
                            class="p-3 rounded-lg text-xs text-center border {messageType ===
                            'success'
                                ? 'bg-green-500/10 border-green-500/20 text-green-200'
                                : 'bg-red-500/10 border-red-500/20 text-red-200'}"
                        >
                            {message}
                        </div>
                    {/if}

                    <div class="flex justify-end">
                        <button
                            type="submit"
                            disabled={loading || !token}
                            class="px-6 py-2.5 bg-white text-black font-bold rounded-xl hover:bg-gray-200 active:scale-[0.98] transition-all disabled:opacity-50 disabled:cursor-not-allowed shadow-lg shadow-glass"
                        >
                            {loading ? "Verifying..." : "Save Changes"}
                        </button>
                    </div>
                </form>
            </section>

            <!-- Danger Zone -->
            <section
                class="bg-glass border border-red-500/20 rounded-3xl p-8 backdrop-blur-xl"
            >
                <div class="flex items-center justify-between">
                    <div>
                        <h2 class="text-lg font-bold text-red-200">
                            Danger Zone
                        </h2>
                        <p class="text-text-secondary text-sm mt-1">
                            Disconnect your account and clear all local data.
                        </p>
                    </div>
                    <button
                        onclick={handleDisconnect}
                        class="px-4 py-2 rounded-xl border text-sm font-medium transition-all active:scale-95 {confirmingDisconnect
                            ? 'bg-red-500 text-white border-red-500'
                            : 'bg-red-500/10 hover:bg-red-500/20 border-red-500/20 text-red-200'}"
                    >
                        {confirmingDisconnect
                            ? "Click to Confirm"
                            : "Disconnect Account"}
                    </button>
                </div>
            </section>

            <!-- About Section -->
            <section
                class="bg-glass border border-border-glass rounded-3xl p-8 backdrop-blur-xl"
            >
                <div class="flex items-center justify-between">
                    <div>
                        <h2 class="text-lg font-bold text-white">About</h2>
                        <p class="text-text-secondary text-sm mt-1">
                            Light Effects v0.3.0
                        </p>
                    </div>
                    <a
                        href="https://github.com/ethan/light-effects"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="p-2.5 rounded-full bg-glass hover:bg-glass-hover border border-border-glass text-text-secondary hover:text-white transition-all"
                        title="View on GitHub"
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
                            ><path
                                d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"
                            /><path d="M9 18c-4.51 2-5-2-7-2" /></svg
                        >
                    </a>
                </div>
            </section>
        </div>
    </main>
</div>
