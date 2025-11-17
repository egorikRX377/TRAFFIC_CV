import { useState } from "react";
import "../assets/login.css";
import { loginUser } from "../api/auth";

export default function Login({ switchToRegister, onLoginSuccess }) {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [remember, setRemember] = useState(false);
  const [message, setMessage] = useState("");
  const [showModal, setShowModal] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();

    try {
      const data = await loginUser({ username, password });

      // Сохраняем токен
      localStorage.setItem("token", data.token);
      console.log("✅ Токен сохранён:", data.token);

      // Показать модалку успешного входа
      setShowModal(true);
      setMessage("Вход успешен! Добро пожаловать 😊");

      // Через 3 секунды закрываем модалку и вызываем переход на Dashboard
      setTimeout(() => {
        setShowModal(false);
        if (onLoginSuccess) onLoginSuccess(); // меняем состояние App
      }, 3000);
    } catch (err) {
      console.error("❌ Ошибка входа:", err);
      setMessage(`Ошибка входа: ${err.message}`);
    }
  };

  return (
    <div className="login-page">
      <div className="col-sm-9 col-md-7 col-lg-5 mx-auto">
        <div className="card border-0 shadow rounded-3 my-5">
          <div className="card-body p-4 p-sm-5">
            <h5 className="card-title text-center mb-5 fw-light fs-5">Sign In</h5>
            {message && !showModal && (
              <p className="text-center text-danger">{message}</p>
            )}

            <form onSubmit={handleSubmit}>
              <div className="form-floating mb-3">
                <input
                  type="text"
                  className="form-control"
                  placeholder="Username"
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  required
                  autoFocus
                />
                <label>Username</label>
              </div>

              <div className="form-floating mb-3">
                <input
                  type="password"
                  className="form-control"
                  placeholder="Password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  required
                />
                <label>Password</label>
              </div>

              <div className="form-check mb-3">
                <input
                  className="form-check-input"
                  type="checkbox"
                  checked={remember}
                  onChange={(e) => setRemember(e.target.checked)}
                />
                <label className="form-check-label">Remember me</label>
              </div>

              <div className="d-grid mb-3">
                <button
                  className="btn btn-primary btn-login text-uppercase fw-bold"
                  type="submit"
                >
                  Sign in
                </button>
              </div>

              <a
                href="#"
                className="d-block text-center mt-3 small"
                onClick={(e) => {
                  e.preventDefault();
                  switchToRegister();
                }}
              >
                Don't have an account? Register
              </a>
            </form>
          </div>
        </div>
      </div>

      {/* 🔹 Модальное окно после успешного входа */}
      {showModal && (
        <div
          className="modal fade show"
          style={{
            display: "block",
            background: "rgba(0,0,0,0.5)",
            position: "fixed",
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
            zIndex: 1050,
          }}
        >
          <div
            className="modal-dialog modal-dialog-centered"
            style={{ maxWidth: "400px" }}
          >
            <div className="modal-content p-4 text-center">
              <h5 className="mb-3">✅ Вход выполнен успешно!</h5>
              <p>Добро пожаловать, {username}!</p>
              <p>Вы будете перенаправлены на Dashboard...</p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
