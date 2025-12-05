import { BACKEND_URL } from "@/lib/export";
import GoogleDrive from '../../../components/GoogleDrives';

export default function Home() {
  return (
    <div>
      <a
        href={`${BACKEND_URL}/auth/drive`}
        className="text-lg border border-stone-700 rounded-2xl p-2.5"
      >
        Add Google Drive
      </a>
      <GoogleDrive />
    </div>
  );
}
