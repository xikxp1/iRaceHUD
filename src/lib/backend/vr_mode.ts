// VR Mode utilities for detecting and configuring VR web overlay mode

import { isOpenKneeboard } from './openkneeboard';

// Check if running inside Tauri
export function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

// Check if running in VR mode (not Tauri)
export function isVrMode(): boolean {
  return !isTauri();
}

// Check if running in OpenKneeBoard
export { isOpenKneeboard } from './openkneeboard';

// Get the VR host (HTTP server origin)
// In VR mode, we use the same host that served the page
// This can be overridden via URL parameters for development
export function getVrHost(): string {
  if (typeof window === 'undefined') {
    return 'http://localhost:8080';
  }

  // Check for override in URL params
  const urlParams = new URLSearchParams(window.location.search);
  const hostOverride = urlParams.get('host');
  if (hostOverride) {
    return hostOverride;
  }

  // Use the current origin (where the page was served from)
  return window.location.origin;
}

// Get the WebSocket host (may be different from HTTP host)
export function getWsHost(): string {
  if (typeof window === 'undefined') {
    return 'localhost';
  }

  // Extract hostname from current URL
  return window.location.hostname;
}
