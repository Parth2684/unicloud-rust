import { ChevronRight } from "lucide-react";
import { Fragment } from "react";

export const Breadcrumb: React.FC<{
  path: Map<string, string>;
  onNavigate: (id: string) => void;
}> = ({ path, onNavigate }) => {
  const entries = Array.from(path.entries()); // [ [id, name], ... ]

  return (
    <div className="flex items-center gap-2 overflow-x-auto pb-2">
      {entries.map(([id, name], index) => (
        <Fragment key={id}>
          <button
            onClick={() => onNavigate(id)}
            className="text-sm font-medium text-gray-700 hover:text-blue-600 whitespace-nowrap"
          >
            {name}
          </button>

          {index < entries.length - 1 && <ChevronRight className="w-4 h-4 text-gray-400" />}
        </Fragment>
      ))}
    </div>
  );
};
