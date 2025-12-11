"use client"

import { useEffect, useState } from "react";
import { DriveFile, SuccessCloudAccount } from "../stores/cloud/types";
import { isFolder } from "../utils/format";
import { ArrowLeft, Folder, Grid3x3, HardDrive, List } from "lucide-react";
import { CloudAccountCard } from "./CloudAccountCard";
import { StorageBar } from "./StorageBar";
import { Breadcrumb } from "./BreadCrumb";
import { FileItem } from "./FileItem";
import { useCloudStore } from '../stores/cloud/useCloudStore';
import { useRouter } from 'next/navigation';
import { BACKEND_URL } from '../lib/export';

export const CloudFileExplorer = () => {
  const [viewMode, setViewMode] = useState<"grid" | "list">("list");
  const [currentView, setCurrentView] = useState<"accounts" | "drive">("accounts");
  const [selectedAccount, setSelectedAccount] = useState<SuccessCloudAccount | null>(null);
  const [breadcrumbPath, setBreadcrumbPath] = useState<Map<string, string>>(new Map());
  // const [files, setFiles] = useState<DriveFile[]>();
  const { setClouds, successCloudAccounts, errorCloudAccounts, setCurrentGoogleFolder, drive } = useCloudStore()
  const handleAccountClick = (account: SuccessCloudAccount) => {
    setSelectedAccount(account);
    setCurrentView("drive");
    setCurrentGoogleFolder(account.info.id, null)
    const newMap = new Map(breadcrumbPath);
    newMap.set(account.info.id, account.info.email);
    setBreadcrumbPath(newMap);
  };

  const handleFileClick = (file: DriveFile, drive_id: string) => {
    if (isFolder(file.mimeType)) {
      const newMap = new Map(breadcrumbPath);
      setCurrentGoogleFolder(drive_id, file.id)
      newMap.set(file.id, file.name);
      setBreadcrumbPath(newMap);
    } else {
      console.log("Open file:", file.name);
    }
  };

  const handleBreadcrumbNavigate = (id: string) => {
    const keys = Array.from(breadcrumbPath.keys());
    const newMap = new Map<string, string>();
    
    for (const key of keys) {
      newMap.set(key, breadcrumbPath.get(key)!);
      if (key === id) break;
    }

    setBreadcrumbPath(newMap);
  };

  const handleBack = () => {
    const keys = Array.from(breadcrumbPath.keys());

    if (keys.length > 1) {
      const parentKey = keys[keys.length - 2]; 
      handleBreadcrumbNavigate(parentKey);
    } else {
      setCurrentView("accounts");
      setSelectedAccount(null);
      setBreadcrumbPath(new Map());
    }
  };
  
  const router = useRouter()
  useEffect(() => {
    setClouds()
  },[])

  const files = drive
  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white border-b border-gray-200 sticky top-0 z-10">
        <div className="max-w-7xl mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              {currentView === "drive" && (
                <button
                  onClick={handleBack}
                  className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
                >
                  <ArrowLeft className="w-5 h-5" />
                </button>
              )}
              <HardDrive className="w-6 h-6 text-blue-600" />
              <h1 className="text-xl font-semibold">UniCloud</h1>
            </div>
            {currentView === "drive" && (
              <div className="flex items-center gap-2">
                <button
                  onClick={() => setViewMode("list")}
                  className={`p-2 rounded-lg transition-colors ${
                    viewMode === "list" ? "bg-blue-100 text-blue-600" : "hover:bg-gray-100"
                  }`}
                >
                  <List className="w-5 h-5" />
                </button>
                <button
                  onClick={() => setViewMode("grid")}
                  className={`p-2 rounded-lg transition-colors ${
                    viewMode === "grid" ? "bg-blue-100 text-blue-600" : "hover:bg-gray-100"
                  }`}
                >
                  <Grid3x3 className="w-5 h-5" />
                </button>
              </div>
            )}
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 py-6">
        {currentView === "accounts" ? (
          <div>
            <h2 className="text-2xl font-semibold mb-6">Cloud Accounts</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {successCloudAccounts && successCloudAccounts.map((account) => (
                <CloudAccountCard
                  key={account.info.id}
                  account={account}
                  onClick={() => handleAccountClick(account)}
                />
              ))}
              
              {errorCloudAccounts && errorCloudAccounts.map((account) => (
                <CloudAccountCard
                  key={account.id}
                  account={account}
                  onClick={() => {
                    router.push(`${BACKEND_URL}/auth/google`)
                  }}
                />
              ))}
            </div>
          </div>
        ) : (
          <div>
            {selectedAccount && (
              <div className="mb-6 p-4 bg-white rounded-lg border border-gray-200">
                <StorageBar account={selectedAccount} />
              </div>
            )}

            {breadcrumbPath.size > 0 && (
              <div className="mb-4">
                <Breadcrumb path={breadcrumbPath} onNavigate={handleBreadcrumbNavigate} />
              </div>
            )}

            <div className="bg-white rounded-lg border border-gray-200 p-4">
              {files && files.length === 0 ? (
                <div className="text-center py-12 text-gray-500">
                  <Folder className="w-16 h-16 mx-auto mb-4 text-gray-300" />
                  <p>This folder is empty</p>
                </div>
              ) : viewMode === "grid" ? (
                <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-2">
                  {selectedAccount && files && files.map((file) => (
                    <FileItem
                      key={file.id}
                      file={file}
                      viewMode={viewMode}
                      onClick={() => handleFileClick(file, selectedAccount.info.id)}
                    />
                  ))}
                </div>
              ) : (
                <div className="space-y-1">
                  {selectedAccount && files &&  files.sort((a,b) => a.name.localeCompare(b.name, undefined, {sensitivity: 'base'})).map((file) => (
                    <FileItem
                      key={file.id}
                      file={file}
                      viewMode={viewMode}
                      onClick={() => handleFileClick(file, selectedAccount.info.id)}
                    />
                  ))}
                </div>
              )}
            </div>
          </div>
        )}
      </main>
    </div>
  );
};
