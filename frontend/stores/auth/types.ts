export interface User {
  id: string;
  username: string;
  email: string;
  image: string;
}

export type AuthState = {
  authUser: User | null;
  isLoggedIn: boolean;
  token: string | null
};

export type AuthAction = {
  setUser: (user: User) => void;
  logout: () => void;
  setToken: () => Promise<void>;
};
