// Single source of truth for the point scale shown to users.
// The actual ranking score is computed server-side (src-tauri/src/commands/ranking.rs),
// which must be kept in sync with these values.
export const POINTS = {
  top1: 3,
  top2: 2,
  top3: 1,
} as const;
