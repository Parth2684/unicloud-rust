import { useState } from "react";
import { Node } from "../stores/cloud/types"


export default function TreeNode({ node }: {node: Node}) {
  const isFolder = node.mimeType === "application/vnd.google-apps.folder";
  const [open, setOpen] = useState(false);

  return (
    <div className="mb-1">
      <div
        className="cursor-pointer flex items-center gap-2"
        onClick={() => isFolder && setOpen(!open)}
      >
        {isFolder ? (open ? "📂" : "📁") : "📄"}
        <span>{node.name}</span>
      </div>

      {open && node.children && (
        <div className="ml-6 border-l pl-2">
          {node.children.length > 0 ? (
            node.children.map(child => (
              <TreeNode key={child.id} node={child} />
            ))
          ) : (
            <div className="text-gray-400">Empty</div>
          )}
        </div>
      )}
    </div>
  );
}
