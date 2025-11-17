import { useState } from "react";
import Login from "./components/Login";
import Register from "./components/Register";
import Dashboard from "./components/Dashboard";

export default function App() {
  const [page, setPage] = useState("register"); // "register" или "login" или "dashboard"

  return (
    <div style={{ fontFamily: "sans-serif", textAlign: "center" }}>
      {page === "register" && <Register switchToLogin={() => setPage("login")} />}
      {page === "login" && <Login switchToRegister={() => setPage("register")} onLoginSuccess={() => setPage("dashboard")} />}
      {page === "dashboard" && <Dashboard />}
    </div>
  );
}
