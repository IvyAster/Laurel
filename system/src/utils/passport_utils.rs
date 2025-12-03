use base64::{Engine, engine::general_purpose::STANDARD};
use rand::RngCore;
use sha1::{Digest, Sha1};

//pub struct Passports;

//impl Passports {
    /// 随机盐生成
    pub fn salt() -> String {
        let mut bytes = [0u8; 16];
        let mut rng = rand::rng();
        rng.fill_bytes(&mut bytes);
        STANDARD.encode(bytes)
    }

    /// 生成秘钥
    pub fn password(account_id: &str, password: &str, salt: &str) -> anyhow::Result<String> {
        let bytes = strings_to_bytes(account_id, password, salt)?;
        let mut hasher = Sha1::new();
        hasher.update(&bytes);
        let result = hasher.finalize();
        Ok(STANDARD.encode(result))
    }

    /// 账户信息组合成字节数组
    fn strings_to_bytes(account_id: &str, password: &str, salt: &str) -> anyhow::Result<Vec<u8>> {
        let b1 = account_id
            .encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect::<Vec<u8>>();
        let b2 = password
            .encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect::<Vec<u8>>();
        let b3 = STANDARD.decode(salt)?;

        let mut bytes = Vec::with_capacity(b1.len() + b2.len() + b3.len());
        bytes.extend_from_slice(&b1);
        bytes.extend_from_slice(&b2);
        bytes.extend_from_slice(&b3);

        Ok(bytes)
    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_salt_length() {
        let salt = salt();
        let decoded = STANDARD.decode(&salt).unwrap();
        assert_eq!(decoded.len(), 16);
    }

    #[test]
    fn test_password_consistency() -> Result<(), Box<dyn std::error::Error>> {
        let salt = "knVrM0F8Pj1nqqPLj9gh4g==".to_string(); //Passports::salt();//"test_salt_base64=="; // 需要是有效的base64
        let account_id = "1958358443601567744";
        let password0 = "110120";

        let result1 = "+ugCaoPg8wHg8iM9caIKGJ4QjDE=".to_string(); // = Passports::password(account_id, password, salt.as_str())?;
        let result2 = password(account_id, password0, salt.as_str())?;
        assert_eq!(result1, result2);
        Ok(())
    }
}
