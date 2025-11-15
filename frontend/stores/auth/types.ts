export interface User {
  id: String;
  username: String;
  email: String;
  image: String;
}

export type AuthState = {
  authUser: User | null;
  isLoggingIn: boolean;
  isLoggingOut: boolean;
  isLoggedIn: boolean;
};

export type AuthAction = {
  login: () => void;
};