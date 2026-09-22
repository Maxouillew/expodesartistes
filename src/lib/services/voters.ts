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
