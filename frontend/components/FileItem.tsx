import { ChevronRight, File, Folder } from "lucide-react";
import { DriveFile } from "../stores/cloud/types";
import { formatBytes, isFolder } from "../utils/format";

export const FileItem: React.FC<{
  file: DriveFile;
  viewMode: "grid" | "list";
  onClick: () => void;
}> = ({ file, viewMode, onClick }) => {
  const folder = isFolder(file.mimeType);

  if (viewMode === "grid") {
    return (
      <button
        onClick={onClick}
        className="flex flex-col items-center p-4 rounded-lg hover:bg-gray-100 transition-colors"
      >
        {folder ? (
          <Folder className="w-12 h-12 text-blue-500 mb-2" fill="currentColor" />
        ) : (
          <File className="w-12 h-12 text-gray-400 mb-2" />
        )}
        <span className="text-sm text-center wrap-break-words w-full">{file.name}</span>
        {file.size && (
          <span className="text-xs text-gray-500 mt-1">{formatBytes(file.size.toString())}</span>
        )}
      </button>
    );
  }

  return (
    <button
      onClick={onClick}
      className="flex items-center gap-3 p-3 rounded-lg hover:bg-gray-100 transition-colors w-full"
    >
      {folder ? (
        <Folder className="w-6 h-6 text-blue-500 shrink-0" fill="currentColor" />
      ) : (
        <File className="w-6 h-6 text-gray-400 shrink-0" />
      )}
      <div className="flex-1 text-left min-w-0">
        <div className="font-medium truncate">{file.name}</div>
        {file.size && <div className="text-xs text-gray-500">{file.size}</div>}
      </div>
      <ChevronRight className="w-4 h-4 text-gray-400 shrink-0" />
    </button>
  );
};
