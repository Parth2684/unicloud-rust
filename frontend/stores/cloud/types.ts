export enum Provider {
  Google ,
  Mega
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
  errorCloudAccounts: ErrorCloudAccount[] | null
}


export type CloudActions = {
  setClouds: () => Promise<void>
}


