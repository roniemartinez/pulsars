import type { Settings } from "@fortune-sheet/core";
import { invoke } from "@tauri-apps/api/core";
import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";
import { FolderOpen, Save } from "lucide-react";
import { xlsxFilters } from "./config";
import { formatError } from "./errors";

type CustomToolbarItems = NonNullable<Settings["customToolbarItems"]>;

type ToolbarActions = {
  onError: (message: string) => void;
  flushOps: () => Promise<void>;
};

const ICON_SIZE = 20;

export function createCustomToolbarItems({ onError, flushOps }: ToolbarActions): CustomToolbarItems {
  const openWorkbook = async () => {
    const filePath = await openDialog({ multiple: false, filters: xlsxFilters });

    if (filePath !== null) {
      await flushOps();
      await invoke("open", { filePath });
    }
  };

  const saveWorkbook = async () => {
    const filePath = await saveDialog({ filters: xlsxFilters });

    if (filePath !== null) {
      await flushOps();
      await invoke("save", { filePath });
    }
  };

  const run = (action: () => Promise<void>) => () => {
    action().catch((cause: unknown) => onError(formatError(cause)));
  };

  return [
    { key: "open", tooltip: "Open", icon: <FolderOpen size={ICON_SIZE} />, onClick: run(openWorkbook) },
    { key: "save", tooltip: "Save", icon: <Save size={ICON_SIZE} />, onClick: run(saveWorkbook) },
  ];
}
