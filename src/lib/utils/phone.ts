// Lightweight check for immediate UX feedback only. The Rust backend
// (src-tauri/src/commands/vote.rs) is the source of truth: it parses the
// number properly (Belgian local or full international format) and
// normalizes it to E.164 before storage.
const ALLOWED_CHARS_PATTERN = /^[0-9 ()./+-]+$/;

export function looksLikePhoneNumber(raw: string): boolean {
  const trimmed = raw.trim();
  if (!ALLOWED_CHARS_PATTERN.test(trimmed)) return false;
  const digitCount = trimmed.replace(/\D/g, "").length;
  return digitCount >= 8 && digitCount <= 15;
}
