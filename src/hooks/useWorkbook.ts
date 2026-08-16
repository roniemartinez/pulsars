import type { Op, Sheet } from "@fortune-sheet/core";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useCallback, useEffect, useState } from "react";
import { formatError } from "../errors";

export type Workbook = {
  sheets: Sheet[] | null;
  error: string | null;
  applyOps: (ops: Op[]) => void;
  reportError: (message: string) => void;
};

export function useWorkbook(): Workbook {
  const [sheets, setSheets] = useState<Sheet[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  // fortune-sheet reads `data` as initial state only, so clearing it is what
  // forces the grid to remount with the newly opened file.
  useEffect(() => {
    const unlisten = listen("reload", () => setSheets(null));

    return () => {
      unlisten.then((stop) => stop()).catch((cause: unknown) => setError(formatError(cause)));
    };
  }, []);

  useEffect(() => {
    if (sheets !== null) {
      return;
    }

    invoke<Sheet[]>("serialize")
      .then((loaded) => {
        setSheets(loaded);
        setError(null);
      })
      .catch((cause: unknown) => setError(formatError(cause)));
  }, [sheets]);

  const applyOps = useCallback((ops: Op[]) => {
    invoke("apply_ops", { ops }).catch((cause: unknown) => setError(formatError(cause)));
  }, []);

  return { sheets, error, applyOps, reportError: setError };
}
