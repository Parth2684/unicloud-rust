import { SuccessCloudAccount } from '../stores/cloud/types';
import { formatBytes, getUsagePercentage } from '../utils/format';



export const StorageBar: React.FC<{ account: SuccessCloudAccount }> = ({ account }) => {
  const { storageQuota } = account;
  const percentage = getUsagePercentage(storageQuota.usage, storageQuota.limit || storageQuota.usage);
  const usedGB = formatBytes(storageQuota.usage);
  const totalGB = storageQuota.limit ? formatBytes(storageQuota.limit) : 'Unlimited';

  return (
    <div className="w-full space-y-2">
      <div className="flex justify-between text-xs text-gray-600">
        <span>{usedGB} used</span>
        <span>{totalGB} total</span>
      </div>
      <div className="w-full bg-gray-200 rounded-full h-2">
        <div
          className={`h-2 rounded-full transition-all ${
            percentage > 90 ? 'bg-red-500' : percentage > 70 ? 'bg-yellow-500' : 'bg-blue-500'
          }`}
          style={{ width: `${Math.min(percentage, 100)}%` }}
        />
      </div>
      <div className="text-xs text-gray-500">{percentage.toFixed(1)}% used</div>
    </div>
  );
};