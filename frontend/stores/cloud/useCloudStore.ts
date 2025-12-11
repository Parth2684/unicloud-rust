import { create } from "zustand";
import { CloudActions, CloudState } from "./types";
import { axiosInstance } from "../../utils/axiosInstance";
import { AxiosError } from "axios";
import toast from "react-hot-toast";

export const useCloudStore = create<CloudState & CloudActions>((set, get) => ({
  loading: false,
  successCloudAccounts: null,
  errorCloudAccounts: null,

  setClouds: async () => {
    set({ loading: true });
    try {
      const res = await axiosInstance.get("/cloud/get-cloud-accounts");
      set({
        successCloudAccounts: res.data.google_drive_accounts,
        errorCloudAccounts: res.data.need_refresh,
      });
    } catch (error) {
      console.error(error);
      if (error instanceof AxiosError && error.response?.data.message) {
        toast.error(error.response.data.message);
      } else {
        toast.error("Unexpected error fetching cloud accounts");
      }
    } finally {
      set({ loading: false });
    }
  },
}));
