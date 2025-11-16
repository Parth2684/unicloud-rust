export interface User {
  id: String;
  username: String;
  email: String;
  image: String;
}

export type AuthState = {
  authUser: User | null;
  isLoggedIn: boolean;
};

export type AuthAction = {
  setUser: (user: User) => void
  logout: () => void
};