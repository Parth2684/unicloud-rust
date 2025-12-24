import { BACKEND_URL } from "@/lib/export";
import { HomeComponent } from "../../../components/HomeComponent";

export default function Home() {
  return (
    <div>
      <div className="m-5">
        <a
          href={`${BACKEND_URL}/auth/drive`}
          className="text-lg border border-stone-700 rounded-2xl p-2.5"
        >
          Add Google Drive
        </a>
      </div>
      <HomeComponent />
    </div>
  );
}
