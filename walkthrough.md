# UI Sync & Refresh Fixes

## Issues Addressed

1. **Refresh Button Broken**: The refresh button appeared to do nothing because the loading spinner was suppressed if data was already loaded.
2. **"All Off" Sync Failure**: Clicking "All Off" would turn off the lights, but the UI would often revert to "On" immediately. This was caused by a race condition where the background poller fetched stale data from the API before it had updated.
3. **Light Cards Out of Sync**: Individual light cards used a disconnected local state, meaning they wouldn't update when the global "All Off" action changed the state.

## Changes

### 1. Fixed Refresh Feedback

Modified `src/lib/lightState.svelte.ts` to always set `loading = true` when refreshing, ensuring the spinner animation plays.

### 2. Paused Polling on Master Toggle

Updated `src/components/Dashboard.svelte` to pause the background polling for 2 seconds (plus the 5s interval) after the "All Off" action. This gives the LIFX API time to catch up before we fetch the status again.

### 3. Reactive Light Cards

Refactored `src/components/LightCard.svelte` to remove its isolated local state (`localPower`). It now uses the reactive `light` prop directly, ensuring it instantly reflects changes made by the Dashboard (like "All Off").

## Verification

- **Build**: Verified that the application builds successfully with `bun tauri build`.
- **Logic Check**:
  - Refresh button now sets `loading` state correctly.
  - "All Off" triggers optimistic update AND pauses polling.
  - Light cards now display the `light.power` from the store, so they will flip to "Off" when the Dashboard updates the store.
