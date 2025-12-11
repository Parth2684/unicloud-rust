import { create } from "zustand";
import { CloudActions, CloudState } from "./types";
import { axiosInstance } from "../../utils/axiosInstance";
import { AxiosError } from "axios";
import toast from "react-hot-toast";

export const useCloudStore = create<CloudState & CloudActions>((set, get) => ({
  loading: false,
  successCloudAccounts: null,
  errorCloudAccounts: null,
  drive: null,

  setClouds: async () => {
    set({ loading: true });
    try {
      const res = await axiosInstance.get("/cloud/get-cloud-accounts");
      set({
        successCloudAccounts: res.data.google_drive_accounts,
        errorCloudAccounts: res.data.need_refresh,
      });
      console.log("success clouds" + get().successCloudAccounts);
      console.log("error clouds" + get().errorCloudAccounts);
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

  setCurrentGoogleFolder: async (drive_id, folder_id) => {
    set({ loading: true });
    try {
      if (!folder_id) {
        const res = await axiosInstance.get(`/cloud/google/root/${drive_id}`);
        set({ drive: res.data.files });
      } else {
        const res = await axiosInstance.get(`/cloud/google/folder/${drive_id}/${folder_id}`);
        set({ drive: res.data.files });
      }
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
