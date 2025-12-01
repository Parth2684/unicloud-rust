import { create } from "zustand";
import { AuthState, AuthAction } from "./types";
import toast from "react-hot-toast";
import { axiosInstance } from '../../utils/axiosInstance';

export const useAuthStore = create<AuthState & AuthAction>((set, get) => ({
  authUser: null,
  isLoggedIn: false,
  token: null,

  setUser: (user) => {
    set({ authUser: user, isLoggedIn: true });
  },

  logout: () => {
    set({ authUser: null, isLoggedIn: false });
  },

  setToken: async () => {
    try {
      const res = await axiosInstance.get("/auth/token")
      if (res.status!== 200) {
        throw new Error("Auth Token not received")
      }
      set({token: res.data.auth_token})
    }
    catch(e){
      console.error(e)
      toast.error(e as string)
    }
  }
}));
