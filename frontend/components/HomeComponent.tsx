"use client";
import { useEffect } from "react";
import { useCloudStore } from "../stores/cloud/useCloudStore";
import { Spinner } from "./ui/spinner";
import Link from "next/link";
import { BACKEND_URL } from "../lib/export";

export const HomeComponent = () => {
  const { setClouds, successCloudAccounts, errorCloudAccounts, loading } = useCloudStore();
  useEffect(() => {
    setClouds();
  }, []);
  return (
    <div>
      {loading ? (
        <div>
          <Spinner />
        </div>
      ) : successCloudAccounts && successCloudAccounts.length > 0 ? (
        successCloudAccounts.map((acc) => {
          return (
            <Link href={`/drive/${acc.info.id}`} key={acc.info.id}>
              {JSON.stringify(acc)}
            </Link>
          );
        })
      ) : (
        <div>No account link</div>
      )}
      {!loading &&
        Array.isArray(errorCloudAccounts) &&
        errorCloudAccounts.length > 0 &&
        errorCloudAccounts.map((acc) => (
          <Link href={`${BACKEND_URL}/auth/drive`} key={acc.id}>
            {JSON.stringify(acc)}
          </Link>
        ))}
    </div>
  );
};
