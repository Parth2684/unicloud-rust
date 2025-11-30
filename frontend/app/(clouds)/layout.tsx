import { ReactNode } from "react";
import WSProvider from "../../components/WSProvider";

export default function CloudLayout({ children }: { children: ReactNode }) {
  return (
    <html>
      <body>
        <WSProvider>{children}</WSProvider>
      </body>
    </html>
  );
}
