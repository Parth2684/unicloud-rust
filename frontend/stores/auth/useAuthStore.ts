import { createStore } from "zustand/vanilla";
import { AuthState, AuthAction } from "./types";
import toast from "react-hot-toast";

export const useAuthStore = createStore<AuthState & AuthAction>((set, get) => ({
  authUser: null,
  isLoggingIn: false,
  isLoggingOut: false,
  isLoggedIn: false,

  login: async () => {
    set({ isLoggingIn: true });
    try {
      window.location.href = `${process.env.BACKEND_URL}/auth/google`;
      const searchParams = new URLSearchParams(window.location.search);
      const error = searchParams.get("error");

      if (error === "email_exists") {
        toast.error(
          "An account with this email already exists. Please sign in."
        );
        window.history.replaceState({}, "", window.location.pathname);
        return true;
      }

      if (error === "oauth_failed") {
        toast.error("Failed to authenticate with Google. Please try again.");
        window.history.replaceState({}, "", window.location.pathname);
        return true;
      }

      if (error === "no_account") {
        toast.error(
          "An account with this email doesn't exist. Please sign up."
        );
        window.history.replaceState({}, "", window.location.pathname);
        return true;
      }

      return false;
    } catch (err) {}
  },
}));
