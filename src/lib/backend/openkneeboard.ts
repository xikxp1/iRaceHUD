// OpenKneeBoard API utilities
// Documentation: https://openkneeboard.com/api/web-dashboards/

// TypeScript interface for OpenKneeBoard API
interface OpenKneeboardAPI {
  SetPreferredPixelSize(width: number, height: number): Promise<void>;
  GetVersion(): Promise<{
    Major: number;
    Minor: number;
    Patch: number;
    Build: number;
    HumanReadable: string;
  }>;
  OpenDeveloperToolsWindow(): Promise<void>;
  EnableExperimentalFeature(name: string, version: number): Promise<void>;
  // Page-based content APIs
  GetPages(): Promise<PageInfo[]>;
  SetPages(pages: PageInfo[]): Promise<void>;
  RequestPageChange(guid: string): Promise<void>;
  SendMessageToPeers(message: unknown): Promise<void>;
  // Event handlers
  addEventListener(event: 'pageChanged', handler: (page: PageInfo) => void): void;
  addEventListener(event: 'pagesChanged', handler: (pages: PageInfo[]) => void): void;
  addEventListener(event: 'peerMessage', handler: (message: unknown) => void): void;
  removeEventListener(event: string, handler: (...args: unknown[]) => void): void;
}

interface PageInfo {
  guid: string;
  pixelSize: { width: number; height: number };
  extraData?: unknown;
}

declare global {
  interface Window {
    OpenKneeboard?: OpenKneeboardAPI;
  }
}

// Check if running inside OpenKneeBoard
export function isOpenKneeboard(): boolean {
  return typeof window !== 'undefined' && 'OpenKneeboard' in window && window.OpenKneeboard !== undefined;
}

// Check user agent for OpenKneeBoard
export function isOpenKneeboardUserAgent(): boolean {
  if (typeof navigator === 'undefined') return false;
  return navigator.userAgent.includes('OpenKneeboard');
}

// Get OpenKneeBoard API (returns null if not available)
export function getOpenKneeboardAPI(): OpenKneeboardAPI | null {
  if (isOpenKneeboard()) {
    return window.OpenKneeboard!;
  }
  return null;
}

// Set preferred pixel size for the overlay
export async function setPreferredPixelSize(width: number, height: number): Promise<boolean> {
  const api = getOpenKneeboardAPI();
  if (!api) return false;

  try {
    // Clamp values to OpenKneeBoard limits (1-16384)
    const clampedWidth = Math.max(1, Math.min(16384, Math.round(width)));
    const clampedHeight = Math.max(1, Math.min(16384, Math.round(height)));
    await api.SetPreferredPixelSize(clampedWidth, clampedHeight);
    return true;
  } catch (error) {
    console.error('OpenKneeBoard SetPreferredPixelSize failed:', error);
    return false;
  }
}

// Get OpenKneeBoard version
export async function getVersion(): Promise<string | null> {
  const api = getOpenKneeboardAPI();
  if (!api) return null;

  try {
    const version = await api.GetVersion();
    return version.HumanReadable;
  } catch (error) {
    console.error('OpenKneeBoard GetVersion failed:', error);
    return null;
  }
}

// Enable page-based content experimental feature
export async function enablePageBasedContent(): Promise<boolean> {
  const api = getOpenKneeboardAPI();
  if (!api) return false;

  try {
    await api.EnableExperimentalFeature('PageBasedContent', 2024073001);
    return true;
  } catch (error) {
    console.error('OpenKneeBoard EnableExperimentalFeature failed:', error);
    return false;
  }
}

// Page-based content helpers
export async function setPages(pages: PageInfo[]): Promise<boolean> {
  const api = getOpenKneeboardAPI();
  if (!api) return false;

  try {
    await api.SetPages(pages);
    return true;
  } catch (error) {
    console.error('OpenKneeBoard SetPages failed:', error);
    return false;
  }
}

export async function requestPageChange(guid: string): Promise<boolean> {
  const api = getOpenKneeboardAPI();
  if (!api) return false;

  try {
    await api.RequestPageChange(guid);
    return true;
  } catch (error) {
    console.error('OpenKneeBoard RequestPageChange failed:', error);
    return false;
  }
}

export function onPageChanged(handler: (page: PageInfo) => void): () => void {
  const api = getOpenKneeboardAPI();
  if (!api) return () => {};

  api.addEventListener('pageChanged', handler);
  return () => api.removeEventListener('pageChanged', handler as (...args: unknown[]) => void);
}

// Generate a stable GUID for an overlay based on its name
export function generateOverlayGuid(overlayName: string): string {
  // Create a simple hash-based GUID from the overlay name
  // This ensures consistent GUIDs across sessions
  const hash = overlayName.split('').reduce((acc, char) => {
    return ((acc << 5) - acc) + char.charCodeAt(0);
  }, 0);

  const hex = Math.abs(hash).toString(16).padStart(8, '0');
  return `iracehud-${overlayName}-${hex}`;
}

export type { PageInfo, OpenKneeboardAPI };
