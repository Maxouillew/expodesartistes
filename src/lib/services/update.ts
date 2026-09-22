import { getVersion } from "@tauri-apps/api/app";
import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

export type { Update };

export function getCurrentVersion(): Promise<string> {
  return getVersion();
}

export function checkForUpdate(): Promise<Update | null> {
  return check();
}

export async function installUpdateAndRelaunch(
  update: Update,
  onProgress?: (downloadedBytes: number, contentLength: number | undefined) => void,
): Promise<void> {
  let downloadedBytes = 0;
  let contentLength: number | undefined;

  await update.downloadAndInstall((event) => {
    if (event.event === "Started") {
      contentLength = event.data.contentLength;
    } else if (event.event === "Progress") {
      downloadedBytes += event.data.chunkLength;
      onProgress?.(downloadedBytes, contentLength);
    }
  });

  await relaunch();
}
