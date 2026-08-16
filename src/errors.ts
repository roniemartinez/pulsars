// Backend errors serialize as { kind, message }.
export function formatError(cause: unknown): string {
  if (typeof cause === "string") {
    return cause;
  }

  if (cause && typeof cause === "object" && "kind" in cause) {
    const { kind, message } = cause as { kind: string; message?: string };
    return message ? `${kind}: ${message}` : kind;
  }

  return String(cause);
}
