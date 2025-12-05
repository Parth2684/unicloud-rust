
export interface Node {
  id: string;
  name: string;
  mimeType: string;
  children: Node[]
}

export interface GoogleDrive {
  drive_name: string;
  files: Record<string, Node>
}



export type CloudState = {
  googleDrives: GoogleDrive[] | null
  loading: boolean
}

export type CloudAction = {
  setGoogleDrives: () => Promise<void>;
}