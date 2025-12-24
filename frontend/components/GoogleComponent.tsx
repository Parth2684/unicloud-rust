"use client";
import { useEffect } from "react";
import Link from "next/link";
import { useCloudStore } from "../stores/cloud/useCloudStore";
import { Spinner } from "./ui/spinner";
import type { SharedDrive } from "../stores/cloud/types";

type GoogleComponentProps = {
  drive_id: string;
};

export const GoogleComponent = ({ drive_id }: GoogleComponentProps) => {
  const { sharedDrives, setSharedDrives, loading } = useCloudStore();

  useEffect(() => {
    setSharedDrives(drive_id);
  }, [drive_id, setSharedDrives]);

  return (
    <div className="flex w-full flex-col gap-8 px-4 py-6 sm:px-8">
      <header className="flex flex-col gap-2">
        <h1 className="text-xl font-semibold text-zinc-900 dark:text-zinc-50 sm:text-2xl">
          Google Drive
        </h1>
        <p className="text-sm text-zinc-500 dark:text-zinc-400">
          Open your personal drive or one of the shared drives linked to this Google account.
        </p>
      </header>

      {loading ? (
        <div className="flex min-h-[160px] items-center justify-center">
          <Spinner />
        </div>
      ) : (
        <>
          <section className="flex flex-col gap-3">
            <h2 className="text-sm font-medium uppercase tracking-wide text-zinc-500 dark:text-zinc-400">
              My Drive
            </h2>
            <Link
              href={`/drive/${drive_id}`}
              className="flex items-center justify-between gap-3 rounded-xl border border-zinc-200 bg-white px-4 py-3 text-sm shadow-sm transition hover:-translate-y-0.5 hover:border-zinc-300 hover:shadow-md dark:border-zinc-800 dark:bg-zinc-900 dark:hover:border-zinc-700"
            >
              <div className="flex items-center gap-3">
                <div className="flex h-8 w-8 items-center justify-center rounded-md bg-sky-500/10 text-sky-600 dark:bg-sky-500/20 dark:text-sky-300">
                  <span className="text-base font-semibold">G</span>
                </div>
                <div className="flex flex-col">
                  <span className="text-sm font-medium text-zinc-900 dark:text-zinc-50">
                    My Drive
                  </span>
                  <span className="text-xs text-zinc-500 dark:text-zinc-400">Personal files</span>
                </div>
              </div>
              <span className="text-xs font-medium text-zinc-500 group-hover:text-zinc-700 dark:text-zinc-400 dark:group-hover:text-zinc-200">
                Open
              </span>
            </Link>
          </section>

          <section className="flex flex-col gap-3">
            <h2 className="text-sm font-medium uppercase tracking-wide text-zinc-500 dark:text-zinc-400">
              Shared drives
            </h2>

            {Array.isArray(sharedDrives) && sharedDrives.length > 0 ? (
              <div className="divide-y divide-zinc-100 overflow-hidden rounded-xl border border-zinc-200 bg-white shadow-sm dark:divide-zinc-800 dark:border-zinc-800 dark:bg-zinc-900">
                {sharedDrives.map((drive) => (
                  <SharedDriveRow key={drive.id} drive={drive} />
                ))}
              </div>
            ) : (
              <div className="rounded-lg border border-dashed border-zinc-300 bg-zinc-50 p-4 text-xs text-zinc-500 dark:border-zinc-700 dark:bg-zinc-900/40 dark:text-zinc-400">
                No shared drives were found for this account.
              </div>
            )}
          </section>
        </>
      )}
    </div>
  );
};

type SharedDriveRowProps = {
  drive: SharedDrive;
};

const SharedDriveRow = ({ drive }: SharedDriveRowProps) => {
  return (
    <Link
      href={`/drive/${drive.id}`}
      className="flex items-center justify-between gap-3 px-4 py-3 text-sm transition hover:bg-zinc-50 dark:hover:bg-zinc-800/60"
    >
      <div className="flex items-center gap-3">
        <div className="flex h-8 w-8 items-center justify-center rounded-md bg-emerald-500/10 text-emerald-600 dark:bg-emerald-500/20 dark:text-emerald-300">
          <span className="text-base font-semibold">S</span>
        </div>
        <span className="truncate text-sm font-medium text-zinc-900 dark:text-zinc-50">
          {drive.name}
        </span>
      </div>
      <span className="text-xs font-medium text-zinc-500 dark:text-zinc-400">Open</span>
    </Link>
  );
};
