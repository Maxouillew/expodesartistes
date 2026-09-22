import { invoke } from "@tauri-apps/api/core";

export interface ArtistWithVotes {
  id: number;
  name: string;
  voteCount: number;
}

export function listArtists(): Promise<ArtistWithVotes[]> {
  return invoke("list_artists");
}

export function addArtist(id: number, name: string): Promise<ArtistWithVotes> {
  return invoke("add_artist", { id, name });
}

export function updateArtist(
  currentId: number,
  newId: number,
  newName: string,
): Promise<ArtistWithVotes> {
  return invoke("update_artist", { currentId, newId, newName });
}

export function deleteArtist(id: number): Promise<void> {
  return invoke("delete_artist", { id });
}
