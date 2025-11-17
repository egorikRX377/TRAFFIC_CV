import { useState } from "react";
import "../assets/register.css";
import { registerUser } from "../api/auth";

export default function Register({ switchToLogin }) {
  const [username, setUsername] = useState("");
  const [fullName, setFullName] = useState("");
  const [email, setEmail] = useState("");
  const [phoneNumber, setPhoneNumber] = useState("");
  const [organization, setOrganization] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [message, setMessage] = useState("");
  const [showModal, setShowModal] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();

    if (password !== confirmPassword) {
      setMessage("Пароли не совпадают");
      return;
    }

    try {
      const data = await registerUser({
        username,
        full_name: fullName,
        email,
        phone_number: phoneNumber,
        organization,
        password,
      });

      console.log("✅ Успешная регистрация:", data);

      // Показать модалку
      setShowModal(true);
      setMessage("Регистрация успешна! Перенаправление...");

      // Через 3 секунды перейти на форму входа
      setTimeout(() => {
        setShowModal(false);
        switchToLogin();
      }, 3000);
    } catch (err) {
      console.error("❌ Ошибка регистрации:", err);
      setMessage(err.message || "Ошибка при регистрации");
    }
  };

  return (
    <div className="register-page">
      <div className="col-lg-10 col-xl-9 mx-auto">
        <div className="card flex-row my-5 border-0 shadow rounded-3 overflow-hidden">
          <div className="card-img-left d-none d-md-flex"></div>
          <div className="card-body p-4 p-sm-5">
            <h5 className="card-title text-center mb-5 fw-light fs-5">Register</h5>
            {message && <p className="text-danger text-center">{message}</p>}

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
                  type="text"
                  className="form-control"
                  placeholder="Full Name"
                  value={fullName}
                  onChange={(e) => setFullName(e.target.value)}
                  required
                />
                <label>Full Name</label>
              </div>

              <div className="form-floating mb-3">
                <input
                  type="email"
                  className="form-control"
                  placeholder="Email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  required
                />
                <label>Email address</label>
              </div>

              <div className="form-floating mb-3">
                <input
                  type="text"
                  className="form-control"
                  placeholder="Phone Number"
                  value={phoneNumber}
                  onChange={(e) => setPhoneNumber(e.target.value)}
                />
                <label>Phone Number</label>
              </div>

              <div className="form-floating mb-3">
                <input
                  type="text"
                  className="form-control"
                  placeholder="Organization"
                  value={organization}
                  onChange={(e) => setOrganization(e.target.value)}
                />
                <label>Organization</label>
              </div>

              <hr />

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

              <div className="form-floating mb-3">
                <input
                  type="password"
                  className="form-control"
                  placeholder="Confirm Password"
                  value={confirmPassword}
                  onChange={(e) => setConfirmPassword(e.target.value)}
                  required
                />
                <label>Confirm Password</label>
              </div>

              <div className="d-grid mb-2">
                <button
                  className="btn btn-lg btn-primary btn-login fw-bold text-uppercase"
                  type="submit"
                >
                  Register
                </button>
              </div>

              <a
                href="#"
                className="d-block text-center mt-2 small"
                onClick={(e) => {
                  e.preventDefault();
                  switchToLogin();
                }}
              >
                Have an account? Sign In
              </a>
            </form>
          </div>
        </div>
      </div>

      {/* 🔹 Простое модальное окно */}
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
              <h5 className="mb-3">✅ Регистрация успешна!</h5>
              <p>Вы будете перенаправлены на страницу входа...</p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
