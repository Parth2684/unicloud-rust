import { create } from 'zustand';
import { CloudAction, CloudState } from './types';
import { axiosInstance } from '../../utils/axiosInstance';
import toast from 'react-hot-toast';


export const useCloudStore = create<CloudState & CloudAction>((set, get) => ({
  loading: false,
  googleDrives: null,
  
  setGoogleDrives: async () => {
    set({ loading: true })
    try {
      const res = await axiosInstance.get("/cloud/get-google-drives")
      set({ googleDrives: res.data.google_drive })
      
    }catch (e) {
      console.error(e);
      toast.error("Error fetching google drives")
    }finally {
      set ({ loading: false })
    }
  },
  
}))