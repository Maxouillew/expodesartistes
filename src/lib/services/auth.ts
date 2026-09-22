import { invoke } from "@tauri-apps/api/core";

export function adminNeedsSetup(): Promise<boolean> {
  return invoke("admin_needs_setup");
}

export function adminSetPassword(password: string): Promise<void> {
  return invoke("admin_set_password", { password });
}

export function adminLogin(password: string): Promise<void> {
  return invoke("admin_login", { password });
}
