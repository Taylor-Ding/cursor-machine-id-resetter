use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// 将名字数据集编译进二进制文件，避免路径问题
const NAMES_DATASET: &str = include_str!("../../names-dataset.txt");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

pub struct EmailGenerator {
    names: Vec<String>,
    domain: String,
}

impl EmailGenerator {
    /// 创建新的邮箱生成器
    pub fn new(domain: String) -> Result<Self, Box<dyn std::error::Error>> {
        // 使用编译进来的名字数据集
        let names: Vec<String> = NAMES_DATASET
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        if names.is_empty() {
            return Err("名字数据集为空".into());
        }
        
        Ok(Self { names, domain })
    }
    
    /// 生成随机名字
    pub fn generate_random_name(&self) -> String {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.names.len());
        self.names[index].clone()
    }
    
    /// 生成随机密码
    pub fn generate_random_password(&self, length: usize) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                  abcdefghijklmnopqrstuvwxyz\
                                  0123456789\
                                  !@#$%^&*";
        let mut rng = rand::thread_rng();
        
        // 确保密码包含至少一个大写字母、一个小写字母、一个数字和一个特殊字符
        let uppercase = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let lowercase = b"abcdefghijklmnopqrstuvwxyz";
        let numbers = b"0123456789";
        let special = b"!@#$%^&*";
        
        let mut password = Vec::new();
        
        // 添加必需字符
        password.push(uppercase[rng.gen_range(0..uppercase.len())] as char);
        password.push(lowercase[rng.gen_range(0..lowercase.len())] as char);
        password.push(numbers[rng.gen_range(0..numbers.len())] as char);
        password.push(special[rng.gen_range(0..special.len())] as char);
        
        // 填充剩余长度
        for _ in 0..(length - 4) {
            let idx = rng.gen_range(0..CHARSET.len());
            password.push(CHARSET[idx] as char);
        }
        
        // 打乱顺序
        use rand::seq::SliceRandom;
        password.shuffle(&mut rng);
        
        password.into_iter().collect()
    }
    
    /// 生成邮箱地址（带时间戳）
    /// 参照 Python 实现：
    /// length = random.randint(0, length)
    /// timestamp = str(int(time.time()))[-length:]
    /// return f"{self.default_first_name}{timestamp}@{self.domain}"
    pub fn generate_email(&self, first_name: &str, timestamp_length: usize) -> String {
        let mut rng = rand::thread_rng();
        // 随机化 length 值（0 到 timestamp_length 之间，包含两端）
        let length = rng.gen_range(0..=timestamp_length);
        
        // 获取当前时间戳字符串
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        
        // Python 的 [-length:] 行为：
        // - 当 length=0 时，[-0:] 返回整个字符串
        // - 当 length>0 时，[-length:] 返回后 length 位
        let suffix = if length == 0 {
            // 特殊情况：length=0 时使用完整时间戳（匹配 Python 的 [-0:] 行为）
            &timestamp
        } else {
            // 取时间戳的后 length 位
            if timestamp.len() >= length {
                &timestamp[timestamp.len() - length..]
            } else {
                &timestamp
            }
        };
        
        format!("{}{}@{}", first_name, suffix, self.domain)
    }
    
    /// 生成完整的账号信息
    pub fn generate_account_info(&self, password_length: Option<usize>, timestamp_length: Option<usize>) -> AccountInfo {
        let first_name = self.generate_random_name();
        let last_name = self.generate_random_name();
        let password = self.generate_random_password(password_length.unwrap_or(16));
        let email = self.generate_email(&first_name, timestamp_length.unwrap_or(4));
        
        AccountInfo {
            email,
            password,
            first_name,
            last_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_password() {
        let generator = EmailGenerator::new("example.com".to_string()).unwrap();
        let password = generator.generate_random_password(16);
        assert_eq!(password.len(), 16);
        
        // 检查是否包含大写字母
        assert!(password.chars().any(|c| c.is_uppercase()));
        // 检查是否包含小写字母
        assert!(password.chars().any(|c| c.is_lowercase()));
        // 检查是否包含数字
        assert!(password.chars().any(|c| c.is_numeric()));
        // 检查是否包含特殊字符
        assert!(password.chars().any(|c| "!@#$%^&*".contains(c)));
    }
    
    #[test]
    fn test_generate_email() {
        let generator = EmailGenerator::new("example.com".to_string()).unwrap();
        let email = generator.generate_email("test", 4);
        assert!(email.ends_with("@example.com"));
        assert!(email.starts_with("test"));
    }
    
    #[test]
    fn test_generate_account_info() {
        let generator = EmailGenerator::new("example.com".to_string()).unwrap();
        let account = generator.generate_account_info(Some(16), Some(4));
        
        assert!(!account.first_name.is_empty());
        assert!(!account.last_name.is_empty());
        assert!(account.email.ends_with("@example.com"));
        assert_eq!(account.password.len(), 16);
    }
}

