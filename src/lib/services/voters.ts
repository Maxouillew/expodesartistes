import { invoke } from "@tauri-apps/api/core";

export interface Voter {
  id: number;
  firstName: string;
  lastName: string;
  phone: string;
  createdAt: string;
}

export function listVoters(): Promise<Voter[]> {
  return invoke("list_voters");
}

export function deleteVoter(id: number): Promise<void> {
  return invoke("delete_voter", { id });
}

/** Deletes every voter and vote (artists are kept). Returns how many voters were removed. */
export function deleteAllVoters(password: string): Promise<number> {
  return invoke("delete_all_voters", { password });
}
