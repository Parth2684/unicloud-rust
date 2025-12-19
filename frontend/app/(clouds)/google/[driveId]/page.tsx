import { GoogleComponent } from "../../../../components/GoogleComponent";

export default async function Google({ params }: { params: Promise<{ driveId: string }> }) {
  const driveId = (await params).driveId;
  return <GoogleComponent drive_id={driveId} />;
}
