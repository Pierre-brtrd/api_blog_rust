pub struct PasswordRequirements {
    pub min_length: usize,
    pub max_length: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_digit: bool,
    pub require_special_char: bool,
}

impl Default for PasswordRequirements {
    fn default() -> Self {
        Self {
            min_length: 8,
            max_length: 255,
            require_uppercase: true,
            require_lowercase: true,
            require_digit: true,
            require_special_char: true,
        }
    }
}

pub fn validate_password(password: &str, req: &PasswordRequirements) -> Result<(), String> {
    if password.len() < req.min_length {
        return Err(format!(
            "Password must be at least {} characters long",
            req.min_length
        ));
    }

    if password.len() > req.max_length {
        return Err(format!(
            "Password must be at most {} characters long",
            req.max_length
        ));
    }

    if req.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
        return Err("Password must contain at least one uppercase letter".to_string());
    }

    if req.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
        return Err("Password must contain at least one lowercase letter".to_string());
    }

    if req.require_digit && !password.chars().any(|c| c.is_ascii_digit()) {
        return Err("Password must contain at least one digit".to_string());
    }

    if req.require_special_char
        && !password
            .chars()
            .any(|c| !c.is_alphanumeric() && !c.is_whitespace())
    {
        return Err("Password must contain at least one special character".to_string());
    }

    Ok(())
}
