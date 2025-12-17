export const formatBytes = (storage: string): string => {
  let bytes = parseInt(storage);
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`;
};

export const getUsagePercentage = (usage: string, limit: string | null): number => {
  if (!limit) return 0;
  return (parseInt(usage) / parseInt(limit)) * 100;
};

export const isFolder = (mimeType: string): boolean => {
  return mimeType === "application/vnd.google-apps.folder";
};
