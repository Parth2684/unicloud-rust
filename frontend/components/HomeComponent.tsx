"use client";
import { useEffect } from "react";
import { useCloudStore } from "../stores/cloud/useCloudStore";
import { Spinner } from "./ui/spinner";
import Link from "next/link";
import { BACKEND_URL } from "../lib/export";
import { Provider, SuccessCloudAccount, ErrorCloudAccount } from "../stores/cloud/types";
import { formatBytes, getUsagePercentage } from "../utils/format";

export const HomeComponent = () => {
  const { setClouds, successCloudAccounts, errorCloudAccounts, loading } = useCloudStore();
  useEffect(() => {
    setClouds();
  }, []);
  return (
    <div className="flex w-full flex-col gap-8 px-4 py-6 sm:px-8">
      <header className="flex flex-col gap-2">
        <h1 className="text-xl font-semibold text-zinc-900 dark:text-zinc-50 sm:text-2xl">
          Cloud accounts
        </h1>
        <p className="text-sm text-zinc-500 dark:text-zinc-400">
          Choose a linked cloud account to open its drive. Storage usage and provider are shown
          below.
        </p>
      </header>

      {loading ? (
        <div className="flex min-h-[160px] items-center justify-center">
          <Spinner />
        </div>
      ) : (
        <>
          <section className="flex flex-col gap-4">
            <h2 className="text-sm font-medium uppercase tracking-wide text-zinc-500 dark:text-zinc-400">
              Linked accounts
            </h2>
            {Array.isArray(successCloudAccounts) && successCloudAccounts.length > 0 ? (
              <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {successCloudAccounts.map((acc) => (
                  <CloudAccountCard key={acc.info.id} account={acc} />
                ))}
              </div>
            ) : (
              <div className="rounded-lg border border-dashed border-zinc-300 bg-zinc-50 p-6 text-sm text-zinc-500 dark:border-zinc-700 dark:bg-zinc-900/40 dark:text-zinc-400">
                No cloud accounts are linked yet.
              </div>
            )}
          </section>

          {Array.isArray(errorCloudAccounts) && errorCloudAccounts.length > 0 && (
            <section className="flex flex-col gap-3">
              <h2 className="text-sm font-medium uppercase tracking-wide text-amber-600 dark:text-amber-400">
                Attention needed
              </h2>
              <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {errorCloudAccounts.map((acc) => (
                  <ErrorAccountCard key={acc.id} account={acc} />
                ))}
              </div>
            </section>
          )}
        </>
      )}
    </div>
  );
};

type CloudAccountCardProps = {
  account: SuccessCloudAccount;
};

const CloudAccountCard = ({ account }: CloudAccountCardProps) => {
  const { info, storageQuota } = account;
  const usage = formatBytes(storageQuota.usageInDrive);
  const limitLabel = storageQuota.limit ? formatBytes(storageQuota.limit) : null;
  const percentage = getUsagePercentage(storageQuota.usage, storageQuota.limit ?? null);

  const providerLabel = info.provider;
  const avatarInitial = info.email?.[0]?.toUpperCase() ?? providerLabel;

  return (
    <Link
      href={`/google/${info.id}`}
      className="group flex flex-col gap-3 rounded-xl border border-zinc-200 bg-white p-4 text-left shadow-sm transition hover:-translate-y-0.5 hover:border-zinc-300 hover:shadow-md dark:border-zinc-800 dark:bg-zinc-900 dark:hover:border-zinc-700"
    >
      <div className="flex items-center gap-3">
        <div className="flex h-10 w-10 items-center justify-center rounded-full bg-zinc-100 text-sm font-semibold text-zinc-700 dark:bg-zinc-800 dark:text-zinc-100">
          <span className="truncate">{avatarInitial}</span>
        </div>
        <div className="flex min-w-0 flex-col">
          <span className="truncate text-sm font-medium text-zinc-900 dark:text-zinc-50">
            {info.email}
          </span>
          <span className="text-xs text-zinc-500 dark:text-zinc-400">{providerLabel}</span>
        </div>
      </div>

      <div className="flex flex-col gap-1">
        <div className="flex items-center justify-between text-xs text-zinc-500 dark:text-zinc-400">
          <span>
            {usage}
            {limitLabel && ` of ${limitLabel}`}
          </span>
          <span>{Math.round(percentage)}%</span>
        </div>
        <div className="h-1.5 overflow-hidden rounded-full bg-zinc-200 dark:bg-zinc-800">
          <div
            className="h-full rounded-full bg-sky-500 transition-all group-hover:bg-sky-400"
            style={{ width: `${Math.min(percentage, 100)}%` }}
          />
        </div>
      </div>
    </Link>
  );
};

type ErrorAccountCardProps = {
  account: ErrorCloudAccount;
};

const ErrorAccountCard = ({ account }: ErrorAccountCardProps) => {
  const providerLabel = account.provider;

  return (
    <Link
      href={`${BACKEND_URL}/auth/drive`}
      className="flex flex-col gap-3 rounded-xl border border-amber-200 bg-amber-50 p-4 text-left text-amber-900 shadow-sm transition hover:-translate-y-0.5 hover:border-amber-300 hover:shadow-md dark:border-amber-700/60 dark:bg-amber-950/40 dark:text-amber-50"
    >
      <div className="flex items-center justify-between gap-2">
        <div className="flex min-w-0 flex-col">
          <span className="truncate text-sm font-medium">{account.email}</span>
          <span className="text-xs opacity-80">{providerLabel}</span>
        </div>
        {account.tokenExpired && (
          <span className="rounded-full bg-amber-100 px-2 py-0.5 text-xs font-medium text-amber-800 dark:bg-amber-900/60 dark:text-amber-100">
            Session expired
          </span>
        )}
      </div>
      <p className="text-xs opacity-80">Tap to reconnect this account.</p>
    </Link>
  );
};
