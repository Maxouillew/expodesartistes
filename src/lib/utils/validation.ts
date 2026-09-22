import { NAME_MAX_LENGTH, NAME_MIN_LENGTH } from "$lib/config/validation";

export function validateName(raw: string): string | undefined {
  const trimmed = raw.trim();
  if (trimmed.length < NAME_MIN_LENGTH || trimmed.length > NAME_MAX_LENGTH) {
    return `Doit contenir entre ${NAME_MIN_LENGTH} et ${NAME_MAX_LENGTH} caractères.`;
  }
  return undefined;
}
