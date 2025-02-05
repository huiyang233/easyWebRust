use tracing::error;
use validator::ValidationError;
use crate::model::sys_file::SysFile;
use crate::service::user_service::UserService;

pub async fn validate_image(file_id: i64) -> Result<(), ValidationError> {
    let image = match SysFile::select_by_id(&file_id).await {
        Ok(image) => {image}
        Err(e) => {
            error!("error:{}",e);
            return Err(ValidationError::new("查询失败"));
        }
    };
    if image.is_none() {
        return Err(ValidationError::new("图片未找到"));
    }

    Ok(())
}

pub async fn validate_user(user_id: i64) -> Result<(), ValidationError> {
    let user =  UserService::get_user_by_id(user_id).await;
    if user.is_none() {
        return Err(ValidationError::new("未找到用户"));
    }
    Ok(())
}

pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    if phone.len() != 11 {
        return Err(ValidationError::new("手机号长度不正确"));
    }
    if !phone.starts_with("1") {
        return Err(ValidationError::new("手机号格式不正确"));
    }
    if !phone.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new("手机号格式不正确"));
    }
    Ok(())
}

pub fn validate_id_card(id_card: &str) -> Result<(), ValidationError> {
    if id_card.len() != 18 {
        return Err(ValidationError::new("身份证号码不正确"));
    }

    let weights = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    let check_codes = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];

    let mut sum = 0;
    for i in 0..17 {
        if let Some(digit) = id_card.chars().nth(i).and_then(|c| c.to_digit(10)) {
            sum += digit as usize * weights[i];
        } else {
            return Err(ValidationError::new("身份证号码不正确"));
        }
    }

    let check_code_index = sum % 11;
    let expected_check_code = check_codes[check_code_index];

    let x = id_card.chars().nth(17).unwrap_or(' ') == expected_check_code;
    if !x {
        return Err(ValidationError::new("身份证号码不正确"));
    }
    Ok(())
}