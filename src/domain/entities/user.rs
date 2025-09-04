use chrono::{DateTime, Utc};

use crate::domain::value_objects::{Email, Password};

pub struct User {
    id: i32,
    name: String,
    email: Email,
    password: Password,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(id: i32, name: String, email: Email, password: Password) -> User {
        let now = Utc::now();

        User {
            id,
            name,
            email,
            password,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &Email {
        &self.email
    }

    pub fn get_password(&self) -> &Password {
        &self.password
    }

    pub fn get_created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn get_updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn get_deleted_at(&self) -> Option<DateTime<Utc>> {
        self.deleted_at
    }

    pub fn update(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn delete(&mut self) {
        self.deleted_at = Some(Utc::now());
    }
}
