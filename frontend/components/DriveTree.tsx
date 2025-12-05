import { useState } from "react";
import TreeNode from "./TreeNode";
import { GoogleDrive } from '../stores/cloud/types';


export default function DriveTree({ drive } : { drive: GoogleDrive }) {
  const [open, setOpen] = useState(true);

  const rootNodes = Object.values(drive.files);

  return (
    <div className="mb-2">
      <div
        className="cursor-pointer font-medium bg-gray-200 p-2 rounded"
        onClick={() => setOpen(!open)}
      >
        📁 {drive.drive_name}
      </div>

      {open && (
        <div className="ml-4 mt-1">
          {rootNodes.map(node => (
            <TreeNode key={node.id} node={node} />
          ))}
        </div>
      )}
    </div>
  );
}
