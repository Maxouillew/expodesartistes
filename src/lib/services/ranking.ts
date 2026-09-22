import { invoke } from "@tauri-apps/api/core";

export interface ArtistRanking {
  rank: number;
  id: number;
  name: string;
  score: number;
  top1Count: number;
  top2Count: number;
  top3Count: number;
}

export interface RankingResponse {
  totalVotes: number;
  artists: ArtistRanking[];
}

export function getRanking(): Promise<RankingResponse> {
  return invoke("get_ranking");
}
