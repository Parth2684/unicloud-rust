import { createStore } from "zustand/vanilla";
import { AuthState, AuthAction } from "./types";
import toast from "react-hot-toast";

export const useAuthStore = createStore<AuthState & AuthAction>((set, get) => ({
  authUser: null,
  isLoggedIn: false,

  setUser: (user) => {
    set({ authUser: user, isLoggedIn: true });
  },

  logout: () => {
    set({ authUser: null, isLoggedIn: false });
  },
}));
