"use client";
import { useEffect } from "react";
import { useCloudStore } from "../stores/cloud/useCloudStore";
import { Spinner } from "./ui/spinner";
import Link from "next/link";

export const GoogleComponent = (drive_id: string) => {
  const { sharedDrives, setSharedDrives, loading } = useCloudStore();
  useEffect(() => {
    setSharedDrives(drive_id);
  }, [drive_id]);
  return (
    <div>
      {loading ? (
        <div>
          <Spinner />
        </div>
      ) : (
        <>
          <Link href={`/drive/${drive_id}`}>My Drive</Link>
          {!loading &&
            sharedDrives &&
            sharedDrives.map((shared) => {
              return (
                <Link key={shared.id} href={`/drive/${shared.id}`}>
                  {JSON.stringify(shared)}
                </Link>
              );
            })}
        </>
      )}
    </div>
  );
};
