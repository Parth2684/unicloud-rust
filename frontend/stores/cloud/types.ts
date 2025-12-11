export enum Provider {
  Google ,
  Mega
}

export interface DriveFile {
  id: string
  name: string
  mimeType: string
  parents?: string[]
  size?: number
  createdTime?: string
  modifiedTime?: string
}

export interface ErrorCloudAccount {
  id: string
  email: string
  provider: Provider
  tokenExpired: boolean
  image?: string
} 

export interface SuccessCloudAccount {
  info: ErrorCloudAccount,
  storageQuota: {
    limit?: number
    usageInDrive: number
    usageInDriveTrash: number
    usage: number
  }
}


export type CloudState = {
  loading: boolean
  successCloudAccounts: SuccessCloudAccount[] | null,
  errorCloudAccounts: ErrorCloudAccount[] | null,
  drive: DriveFile[] | null
}


export type CloudActions = {
  setClouds: () => Promise<void>
  setCurrentGoogleFolder: (drive_id: string, folder_id?: string) => Promise<void>
}


