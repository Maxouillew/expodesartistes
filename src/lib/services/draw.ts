import { invoke } from "@tauri-apps/api/core";

export interface DrawnVoter {
  id: number;
  firstName: string;
  lastName: string;
  phone: string;
}

export function drawVoter(): Promise<DrawnVoter> {
  return invoke("draw_voter");
}

export function resetDraw(): Promise<void> {
  return invoke("reset_draw");
}
