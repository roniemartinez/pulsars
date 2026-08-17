import type { Op, Sheet } from "@fortune-sheet/core";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useCallback, useEffect, useRef, useState } from "react";
import { formatError } from "../errors";

export type Workbook = {
  sheets: Sheet[] | null;
  revision: number;
  loadError: string | null;
  actionError: string | null;
  applyOps: (ops: Op[]) => void;
  flushOps: () => Promise<void>;
  reportError: (message: string) => void;
  dismissError: () => void;
};

export function useWorkbook(): Workbook {
  const [sheets, setSheets] = useState<Sheet[] | null>(null);
  const [revision, setRevision] = useState(0);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);

  useEffect(() => {
    const unlisten = listen("reload", () => {
      setSheets(null);
      setLoadError(null);
      setRevision((current) => current + 1);
    });

    return () => {
      unlisten.then((stop) => stop()).catch((cause: unknown) => setActionError(formatError(cause)));
    };
  }, []);

  // biome-ignore lint/correctness/useExhaustiveDependencies: revision is a re-run signal, not a value read here
  useEffect(() => {
    let active = true;

    invoke<Sheet[]>("serialize")
      .then((loaded) => {
        if (!active) {
          return;
        }
        setSheets(loaded);
        setLoadError(null);
      })
      .catch((cause: unknown) => {
        if (!active) {
          return;
        }
        setLoadError(formatError(cause));
      });

    return () => {
      active = false;
    };
  }, [revision]);

  const queue = useRef<Promise<void>>(Promise.resolve());

  const applyOps = useCallback((ops: Op[]) => {
    queue.current = queue.current
      .then(() => invoke<void>("apply_ops", { ops }))
      .catch((cause: unknown) => setActionError(formatError(cause)));
  }, []);

  const flushOps = useCallback(() => queue.current, []);

  const dismissError = useCallback(() => setActionError(null), []);

  return {
    sheets,
    revision,
    loadError,
    actionError,
    applyOps,
    flushOps,
    reportError: setActionError,
    dismissError,
  };
}
