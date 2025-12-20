"use client";
import { useEffect } from "react";
import Link from "next/link";
import { useCloudStore } from "../stores/cloud/useCloudStore";
import { Spinner } from "./ui/spinner";
import type { DriveFile } from "../stores/cloud/types";
import { formatBytes, isFolder } from "../utils/format";

type DriveComponentProps = {
  drive_id: string;
  folder_id?: string;
};

export const DriveComponent = ({ drive_id, folder_id }: DriveComponentProps) => {
  const { setCurrentGoogleFolder, drive, loading } = useCloudStore();

  useEffect(() => {
    setCurrentGoogleFolder(drive_id, folder_id);
  }, [drive_id, folder_id, setCurrentGoogleFolder]);

  const title = folder_id ? "Folder" : "My Drive";

  return (
    <div className="flex h-[calc(100vh-4rem)] flex-col gap-4 px-4 py-4 sm:px-8 sm:py-6">
      <header className="flex flex-col gap-1">
        <h1 className="text-xl font-semibold text-zinc-900 dark:text-zinc-50 sm:text-2xl">{title}</h1>
        <p className="text-sm text-zinc-500 dark:text-zinc-400">
          Browse your files and folders. Click a folder name to open it.
        </p>
      </header>

      <div className="flex items-center justify-between gap-3 text-xs text-zinc-500 dark:text-zinc-400">
        <div className="flex items-center gap-1">
          <Link href={`/drive/${drive_id}`} className="font-medium text-zinc-700 hover:underline dark:text-zinc-200">
            My Drive
          </Link>
          {folder_id && <span className="text-zinc-400">/</span>}
          {folder_id && <span className="truncate">Current folder</span>}
        </div>
      </div>

      <div className="flex min-h-0 flex-1 flex-col overflow-hidden rounded-xl border border-zinc-200 bg-white shadow-sm dark:border-zinc-800 dark:bg-zinc-900">
        {loading ? (
          <div className="flex flex-1 items-center justify-center">
            <Spinner />
          </div>
        ) : Array.isArray(drive) && drive.length > 0 ? (
          <FileTable driveId={drive_id} items={drive} />
        ) : (
          <div className="flex flex-1 items-center justify-center px-4 py-8 text-sm text-zinc-500 dark:text-zinc-400">
            This folder is empty.
          </div>
        )}
      </div>
    </div>
  );
};

type FileTableProps = {
  driveId: string;
  items: DriveFile[];
};

const FileTable = ({ driveId, items }: FileTableProps) => {
  return (
    <div className="flex h-full flex-col text-sm">
      <div className="grid grid-cols-[minmax(0,3fr)_minmax(0,1fr)_minmax(0,1fr)] items-center gap-2 border-b border-zinc-200 bg-zinc-50 px-4 py-2 text-xs font-medium uppercase tracking-wide text-zinc-500 dark:border-zinc-800 dark:bg-zinc-900/60 dark:text-zinc-400">
        <span>Name</span>
        <span className="text-right sm:text-left sm:pl-4">Size</span>
        <span className="hidden text-right sm:block">Modified</span>
      </div>
      <div className="flex-1 overflow-auto">
        {items.map((item) => (
          <FileRow key={item.id} driveId={driveId} item={item} />
        ))}
      </div>
    </div>
  );
};

type FileRowProps = {
  driveId: string;
  item: DriveFile;
};

const FileRow = ({ driveId, item }: FileRowProps) => {
  const folder = isFolder(item.mimeType);
  const sizeLabel = !folder && item.size != null ? formatBytes(item.size.toString()) : "--";
  const modifiedDate = item.modifiedTime || item.createdTime;
  const modifiedLabel = modifiedDate ? new Date(modifiedDate).toLocaleDateString() : "--";

  const rowContent = (
    <div className="grid grid-cols-[minmax(0,3fr)_minmax(0,1fr)_minmax(0,1fr)] items-center gap-2 px-4 py-2 text-sm text-zinc-800 hover:bg-zinc-50 dark:text-zinc-100 dark:hover:bg-zinc-800/60">
      <div className="flex items-center gap-2 truncate">
        <div
          className={`flex h-6 w-6 items-center justify-center rounded ${
            folder
              ? "bg-sky-500/10 text-sky-600 dark:bg-sky-500/20 dark:text-sky-300"
              : "bg-zinc-200/60 text-zinc-600 dark:bg-zinc-700 dark:text-zinc-200"
          }`}
        >
          <span className="text-xs font-semibold">{folder ? "F" : "•"}</span>
        </div>
        <span className="truncate">{item.name}</span>
      </div>
      <div className="text-right text-xs text-zinc-500 sm:text-left sm:pl-4 dark:text-zinc-400">{sizeLabel}</div>
      <div className="hidden text-right text-xs text-zinc-500 sm:block dark:text-zinc-400">{modifiedLabel}</div>
    </div>
  );

  if (folder) {
    return (
      <Link href={`/drive/${driveId}/${item.id}`} className="block">
        {rowContent}
      </Link>
    );
  }

  return rowContent;
};
