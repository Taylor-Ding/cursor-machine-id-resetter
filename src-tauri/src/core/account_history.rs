use crate::core::email_generator::AccountInfo;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountHistoryItem {
    pub id: String,
    pub account_info: AccountInfo,
    pub created_at: DateTime<Utc>,
    pub domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountHistory {
    items: Vec<AccountHistoryItem>,
}

impl AccountHistory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, account_info: AccountInfo, domain: String) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let item = AccountHistoryItem {
            id: id.clone(),
            account_info,
            created_at: Utc::now(),
            domain,
        };
        self.items.push(item);
        id
    }

    pub fn get_items(&self) -> &Vec<AccountHistoryItem> {
        &self.items
    }

    pub fn delete_item(&mut self, id: &str) -> Result<(), String> {
        let index = self
            .items
            .iter()
            .position(|item| item.id == id)
            .ok_or_else(|| "未找到指定的历史记录".to_string())?;
        self.items.remove(index);
        Ok(())
    }

    pub fn clear_all(&mut self) {
        self.items.clear();
    }
}

pub struct AccountHistoryManager {
    history_file: PathBuf,
}

impl AccountHistoryManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let app_data_dir = if cfg!(target_os = "windows") {
            std::env::var("APPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
        } else if cfg!(target_os = "macos") {
            dirs::home_dir()
                .map(|p| p.join("Library").join("Application Support"))
                .unwrap_or_else(|| PathBuf::from("."))
        } else {
            dirs::home_dir()
                .map(|p| p.join(".config"))
                .unwrap_or_else(|| PathBuf::from("."))
        };

        let app_dir = app_data_dir.join("cursor-machine-id-resetter");
        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)?;
        }

        let history_file = app_dir.join("account_history.json");

        Ok(Self { history_file })
    }

    pub fn load_history(&self) -> Result<AccountHistory, Box<dyn std::error::Error>> {
        if !self.history_file.exists() {
            return Ok(AccountHistory::new());
        }

        let content = fs::read_to_string(&self.history_file)?;
        let history: AccountHistory = serde_json::from_str(&content)?;
        Ok(history)
    }

    pub fn save_history(&self, history: &AccountHistory) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(history)?;
        fs::write(&self.history_file, content)?;
        Ok(())
    }

    pub fn add_account(
        &self,
        account_info: AccountInfo,
        domain: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut history = self.load_history()?;
        let id = history.add_item(account_info, domain);
        self.save_history(&history)?;
        Ok(id)
    }

    pub fn get_all_accounts(&self) -> Result<Vec<AccountHistoryItem>, Box<dyn std::error::Error>> {
        let history = self.load_history()?;
        let mut items = history.get_items().clone();
        // 按时间倒序排列（最新的在前面）
        items.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(items)
    }

    pub fn delete_account(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut history = self.load_history()?;
        history.delete_item(id)?;
        self.save_history(&history)?;
        Ok(())
    }

    pub fn clear_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut history = self.load_history()?;
        history.clear_all();
        self.save_history(&history)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_history() {
        let mut history = AccountHistory::new();
        
        let account = AccountInfo {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        
        let id = history.add_item(account.clone(), "example.com".to_string());
        assert_eq!(history.get_items().len(), 1);
        
        let _ = history.delete_item(&id);
        assert_eq!(history.get_items().len(), 0);
    }
}

