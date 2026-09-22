import { invoke } from "@tauri-apps/api/core";

export interface PublicArtist {
  id: number;
  name: string;
}

export interface VoteInput {
  firstName: string;
  lastName: string;
  phone: string;
  top1Id: number;
  top2Id: number;
  top3Id: number;
}

export function listArtistsForVote(): Promise<PublicArtist[]> {
  return invoke("list_artists_for_vote");
}

export function submitVote(input: VoteInput): Promise<void> {
  return invoke("submit_vote", { ...input });
}
