//! locales

use std::collections::HashMap;

type TranslationMap = HashMap<&'static str, &'static str>;

fn en() -> TranslationMap {
  let mut m = HashMap::new();
  m.insert("USER_EXIST", "USER_EXIST");
  m.insert("USER_NOT_EXIST", "USER_NOT_EXIST");
  m.insert("USER_REGISTERED", "USER_REGISTERED");
  m.insert("TOKEN_EXPIRED", "TOKEN_EXPIRED");
  m.insert(
    "TWO_FACTOR_AUTH_ERROR_DETAIL",
    "TWO_FACTOR_AUTH_ERROR_DETAIL",
  );
  m.insert("Unauthorized", "Unauthorized");
  m.insert(
    "Registration Confirm Mail",
    "[{name}] Registration Confirm Mail",
  );
  m.insert("confirm registration", "Please click <a href=\"{url}\">{url}<a/> to confirm registration, the link is valid for 1 hour. If you are not registering, please ignore this email.");
  m.insert("Registration confirm mail send failed", "Registration confirm mail send failed, please {%- if isAdmin -%}check your mail configuration{%- else -%}check your email address and contact administrator{%- endif -%}.");
  m
}

fn zh_cn() -> TranslationMap {
  let mut m = HashMap::new();
  m.insert("USER_EXIST", "用户已存在");
  m.insert("USER_NOT_EXIST", "用户不存在");
  m.insert("USER_REGISTERED", "用户已注册");
  m.insert("TOKEN_EXPIRED", "密钥已过期");
  m.insert("TWO_FACTOR_AUTH_ERROR_DETAIL", "二步验证失败");
  m.insert("Unauthorized", "没有授权");
  m.insert("Registration Confirm Mail", "【{name}】注册确认邮件");
  m.insert("confirm registration", "请点击 <a href='{url}'>{url}</a> 确认注册，链接有效时间为 1 个小时。如果不是你在注册，请忽略这封邮件。");
  m.insert("Registration confirm mail send failed", "注册确认邮件发送失败，请{%- if isAdmin -%}检查一下网站的邮件相关配置{% else %}确认你的邮箱输入无误并联系管理员{%- endif -%}。");
  m
}

fn zh_tw() -> TranslationMap {
  let mut m = HashMap::new();
  m.insert("USER_EXIST", "用戶已存在");
  m.insert("USER_NOT_EXIST", "用戶不存在");
  m.insert("USER_REGISTERED", "用戶已註冊");
  m.insert("TOKEN_EXPIRED", "密鑰已過期");
  m.insert("TWO_FACTOR_AUTH_ERROR_DETAIL", "二步驗證失敗");
  m.insert("Unauthorized", "Unauthorized");
  m.insert("Registration Confirm Mail", "『{name}』註冊確認郵件");
  m.insert("confirm registration", "請點擊 <a href=\"{url}\">{url}</a> 確認註冊，鏈接有效時間為 1 個小時。如果不是你在註冊，請忽略這封郵件。");
  m.insert("Registration confirm mail send failed", "註冊確認郵件發送失敗，{%- if isAdmin -%}檢查一下網站的郵件相關配置{% else %}確認你的郵箱輸入無誤後聯繫管理員{%- endif -%}。");
  m
}

/// Gets the corresponding text translation according to lang (Default in English)
pub fn get_translation(lang: &str, key: &str) -> String {
  let translations = match lang {
    "zh" | "zh-cn" | "zh-CN" => zh_cn(),
    "zh-tw" | "zh-TW" => zh_tw(),
    "en" | "en-us" | "en-US" => en(),
    _ => en(),
  };

  translations.get(key).copied().unwrap_or(key).to_string()
}
