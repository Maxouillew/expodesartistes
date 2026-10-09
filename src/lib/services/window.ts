import { getCurrentWindow } from "@tauri-apps/api/window";
import { exit } from "@tauri-apps/plugin-process";

const FULLSCREEN_KEY = "fullscreen";

export function isFullscreen(): Promise<boolean> {
  return getCurrentWindow().isFullscreen();
}

/** Toggles fullscreen and remembers the choice for the next launch. */
export async function setFullscreen(enabled: boolean): Promise<void> {
  await getCurrentWindow().setFullscreen(enabled);
  try {
    localStorage.setItem(FULLSCREEN_KEY, String(enabled));
  } catch {
    // Persistence is a convenience only.
  }
}

/** Re-applies the fullscreen preference saved by a previous session, if any. */
export async function restoreFullscreen(): Promise<void> {
  try {
    if (localStorage.getItem(FULLSCREEN_KEY) === "true") {
      await getCurrentWindow().setFullscreen(true);
    }
  } catch {
    // Not running inside Tauri, or storage unavailable: stay windowed.
  }
}

export function quitApp(): Promise<void> {
  return exit(0);
}
