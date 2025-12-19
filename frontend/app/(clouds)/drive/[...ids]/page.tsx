import { DriveComponent } from "../../../../components/DriveComponent";

export default async function Drive({ params }: { params: Promise<{ ids: string[] }> }) {
  const ids = (await params).ids;
  const driveId = ids[0];
  const folderId = ids[1] ?? undefined;
  return <DriveComponent drive_id={driveId} folder_id={folderId} />;
}
