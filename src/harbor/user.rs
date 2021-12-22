use super::client::Client;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::{Method, StatusCode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    email: String,
    username: String,
    realname: String,
    password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_admin_role: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reset_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSearch {
    pub username: String,
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub action: String,
    pub resource: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub comment: String,
    pub email: String,
    pub realname: String
}

impl User {
    pub fn new(email: String, username: String, realname: String, password: String, comment: Option<String>) -> Self {
        User {
            email,
            username,
            realname,
            password,
            comment,
            update_time: None,
            user_id: None,
            deleted: None,
            creation_time: None,
            role_id: None,
            has_admin_role: None,
            role_name: None,
            reset_uuid: None,
            password_version: None,
        }
    }

    pub fn get_user_profile(&self) -> UserProfile {
        let comment;
        if let Some(c) = &self.comment {
            comment = c.to_string();
        } else {
            comment = String::new();
        }
        UserProfile {
            comment,
            email: self.email.clone(),
            realname: self.realname.clone()
        }
    }
}

impl Client {
    /// Get user with user id.
    pub async fn get_user(&self, id: i64) -> Result<User> {
        let path = format!("/users/{}", id);
        let resp = self.build_request(Method::GET, path).send().await?;
        if resp.status().eq(&StatusCode::OK) {
            Ok(resp.json::<User>().await?)
        } else {
            Err(anyhow!("failed to get user: {}", resp.text().await?))
        }
    }

    /// Get current user info.
    pub async fn get_current_user(&self) -> Result<User> {
        let path = "/users/current";
        let resp = self.build_request(Method::GET, path).send().await?;
        if resp.status().eq(&StatusCode::OK) {
            Ok(resp.json::<User>().await?)
        } else {
            Err(anyhow!("failed to get current user: {}", resp.text().await?))
        }
    }

    /// Mark a registered user as be removed.
    pub async fn delete_user(&self, id: i64) -> Result<()> {
        let path = format!("/users/{}", id);
        let resp = self.build_request(Method::DELETE, path).send().await?;
        if resp.status().eq(&StatusCode::OK) {
            Ok(())
        } else {
            Err(anyhow!("failed to delete user: {}", resp.text().await?))
        }
    }

    /// Get registered users of Harbor.
    pub async fn list_users(&self, username: Option<String>, email: Option<String>, page: Option<u32>, page_size: Option<u32>) -> Result<Vec<User>> {
        let path = "/users";
        let mut params = Vec::new();
        if let Some(username) = username {
            params.push(("username", username));
        }
        if let Some(email) = email {
            params.push(("email", email));
        }
        if let Some(page) = page {
            params.push(("page", page.to_string()));
        }
        if let Some(page_size) = page_size {
            params.push(("page_size", page_size.to_string()));
        }
        let resp = self.build_request(Method::GET, path).query(&params).send().await?;
        if resp.status().eq(&StatusCode::OK) {
            Ok(resp.json::<Vec<User>>().await?)
        } else {
            Err(anyhow!("failed to list users: {}", resp.text().await?))
        }

    }

    /// Creates a new user account.
    pub async fn create_user(&self, user: &User) -> Result<()> {
        let path = "/users";
        let resp = self.build_request(Method::POST, path).json(user).send().await?;
        if resp.status().eq(&StatusCode::CREATED) {
            Ok(())
        } else {
            Err(anyhow!("failed to create user: {}", resp.text().await?))
        }
    }

    /// Search users by username
    pub async fn search_users(&self, username: &str, page: Option<u32>, page_size: Option<u32>) -> Result<Vec<UserSearch>> {
        let path = "/users/search";
        let mut params = vec![("username", username.to_string())];
        if let Some(page) = page {
            params.push(("page", page.to_string()));
        }
        if let Some(page_size) = page_size {
            params.push(("page_size", page_size.to_string()));
        }
        let resp = self.build_request(Method::GET, path).query(&params).send().await?;
        if resp.status().eq(&StatusCode::OK) {
            Ok(resp.json::<Vec<UserSearch>>().await?)
        } else {
            Err(anyhow!("failed to search users: {}", resp.text().await?))
        }

    }

    /// Get current user permissions.
    pub async fn list_current_user_permissions(&self) -> Result<Vec<Permission>> {
        let path = "/users/current/permissions";
        let resp = self.build_request(Method::GET, path).send().await?;
        if resp.status().eq(&StatusCode::OK) {
            Ok(resp.json::<Vec<Permission>>().await?)
        } else {
            Err(anyhow!("failed to list current user permissions: {}", resp.text().await?))
        }

    }

    /// Update a registered user to change his profile.
    pub async fn update_user_profile(&self, id: i64, profile: &UserProfile) -> Result<()> {
        let path = format!("/users/{}", id);
        let resp = self.build_request(Method::PUT, path).json(profile).send().await?;
        if resp.status().eq(&StatusCode::OK) {
            Ok(())
        } else {
            Err(anyhow!("failed to update user profile: {}", resp.text().await?))
        }
    }

    /// Update a registered user to change to be an administrator of Harbor.
    pub async fn update_sysadmin(&self, id: i64, has_admin_role: bool) -> Result<()> {
        let path = format!("/users/{}/sysadmin", id);
        let body = json!({ "has_admin_role": has_admin_role });
        let resp = self.build_request(Method::PUT, path).json(&body).send().await?;
        if resp.status().eq(&StatusCode::OK) {
            Ok(())
        } else {
            Err(anyhow!("failed to update sysadmin: {}", resp.text().await?))
        }
    }

    /// Change the password on a user that already exists.
    pub async fn update_password(&self, id: i64, new_password: &str) -> Result<()> {
        let path = format!("/users/{}/password", id);
        let body = json!({ "new_password": new_password });
        let resp = self.build_request(Method::PUT, path).json(&body).send().await.unwrap();
        if resp.status().eq(&StatusCode::OK) {
            Ok(())
        } else {
            Err(anyhow!("failed to update password: {}", resp.text().await?))
        }
    }

    /// Set CLI secret for a user.
    pub async fn update_cli_secret(&self, id: i64, secret: &str) -> Result<()> {
        let path = format!("/users/{}/cli_secret", id);
        let payload = json!({ "secret": secret });
        let resp = self.build_request(Method::PUT, path).json(&payload).send().await?;
        if resp.status().eq(&StatusCode::OK) {
            Ok(())
        } else {
            Err(anyhow!("failed to update cli secret: {}", resp.text().await?))
        }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn get_current_user() {
        let client = crate::Client::setup().unwrap();
        let user = client.get_current_user().await.unwrap();
        assert_eq!(user.user_id.unwrap(), 1);
    }

    #[tokio::test]
    async fn get_user_profile() {
        let client = crate::Client::setup().unwrap();
        let user = client.get_current_user().await.unwrap();
        let user_profile = user.get_user_profile();
        assert_eq!(user_profile.email, user_profile.email);
    }

    #[tokio::test]
    async fn create_user() {
        let client = crate::Client::setup().unwrap();
        let user = super::User::new(
            String::from("testuser@gmail.com"),
            String::from("testuser"),
            String::from("testuser"),
            String::from("Test1234"),
            Some(String::from("testuser")));
        client.create_user(&user).await.unwrap();
    }

    #[tokio::test]
    async fn list_users() {
        let client = crate::Client::setup().unwrap();
        let users = client.list_users(Some(String::from("testuser")), None, None, None).await.unwrap();
        assert!(users.len() > 0);
    }

    #[tokio::test]
    async fn search_users() {
        let client = crate::Client::setup().unwrap();
        let users = client.search_users("testuser", None, None).await.unwrap();
        assert!(users.len() > 0);
    }

    #[tokio::test]
    async fn update_user_profile() {
        let client = crate::Client::setup().unwrap();
        let users = client.list_users(None, Some(String::from("testuser@gmail.com")), None, None).await.unwrap();
        let user = users.first().unwrap();
        let mut user_profile = user.get_user_profile();
        user_profile.comment = String::from("updated comment");
        client.update_user_profile(user.user_id.unwrap(), &user_profile).await.unwrap();
    }

    #[tokio::test]
    async fn delete_user() {
        let client = crate::Client::setup().unwrap();
        let users = client.list_users(None, Some(String::from("testuser@gmail.com")), None, None).await.unwrap();
        let user = users.first().unwrap();
        client.delete_user(user.user_id.unwrap()).await.unwrap();
    }

    #[tokio::test]
    async fn list_current_user_permissions() {
        let client = crate::Client::setup().unwrap();
        client.list_current_user_permissions().await.unwrap();
    }
}