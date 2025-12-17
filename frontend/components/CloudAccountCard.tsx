"use client";

import { ChevronRight, Cloud } from "lucide-react";
import { ErrorCloudAccount, SuccessCloudAccount } from "../stores/cloud/types";
import { StorageBar } from "./StorageBar";

type Account = SuccessCloudAccount | ErrorCloudAccount;

export const CloudAccountCard: React.FC<{
  account: Account;
  onClick: () => void;
}> = ({ account, onClick }) => {
  return (
    <button
      onClick={onClick}
      className="w-full p-4 bg-white border border-gray-200 rounded-lg hover:border-blue-500 hover:shadow-md transition-all text-left"
    >
      <div className="flex items-start gap-3">
        <div className="p-2 bg-blue-50 rounded-lg">
          {"info" in account ? (
            account.info.image ? (
              <img src={account.info.image} className="w-6 h-6" />
            ) : (
              <Cloud className="w-6 h-6 text-blue-600" />
            )
          ) : (
            <Cloud className="w-6 h-6 text-blue-600" />
          )}
        </div>
        <div className="flex-1 min-w-0">
          {"info" in account ? (
            <h3 className="font-semibold text-gray-900 truncate">{account.info.email}</h3>
          ) : (
            <h3 className="font-semibold text-gray-900 truncate">{account.email}</h3>
          )}
          <p className="text-sm text-gray-500 mb-3">Google Drive</p>
          {"info" in account ? (
            <StorageBar account={account} />
          ) : (
            <p className="text-red-600">Account Expired Please Tap To Give Access Again</p>
          )}
        </div>
        <ChevronRight className="w-5 h-5 text-gray-400 shrink-0" />
      </div>
    </button>
  );
};
