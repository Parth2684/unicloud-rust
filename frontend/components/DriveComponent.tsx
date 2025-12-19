"use client";
import { useEffect } from "react";
import { useCloudStore } from "../stores/cloud/useCloudStore";
import Link from "next/link";
import { Spinner } from "./ui/spinner";

type DriveComponentProps = {
  drive_id: string;
  folder_id?: string;
};

export const DriveComponent = (ids: DriveComponentProps) => {
  const { setCurrentGoogleFolder, drive, loading } = useCloudStore();
  useEffect(() => {
    setCurrentGoogleFolder(ids.drive_id, ids.folder_id);
  }, [drive]);

  return (
    <>
      {!loading &&
        drive &&
        drive.map((file) => {
          <Link id={file.id} href={`/drive/${ids.drive_id}/${ids.folder_id}`}>
            {JSON.stringify(file)}
          </Link>;
        })}
      {loading && <Spinner />}
    </>
  );
};
