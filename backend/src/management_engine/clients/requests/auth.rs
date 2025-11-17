pub const INSERT_USER: &str =
    "INSERT INTO users (username, password_hash, role_id, user_info_id) VALUES ($1, $2, $3, $4)";
pub const SELECT_USER_DETAILS: &str = "SELECT u.password_hash, r.role_name FROM users u JOIN roles r ON u.role_id = r.id WHERE u.username = $1";
pub const SELECT_ROLE_ID_BY_NAME: &str = "SELECT id FROM roles WHERE role_name = $1";

pub const INSERT_USER_INFO: &str = r#"
INSERT INTO user_info (full_name, email, phone_number, organization)
VALUES ($1, $2, $3, $4)
RETURNING id
"#;
