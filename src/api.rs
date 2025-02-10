use utoipa::openapi::{Contact, OpenApi};

pub fn modify_api(api: &mut OpenApi) {
  api.info.title = "actixweb-seaorm-openapi-template".to_string();
  api.info.description =
    Some("A template project for actix-web, sea-orm, openapi, utoipa".to_string());
  let mut contact = Contact::new();
  contact.name = Some("JQiue".to_string());
  contact.url = Some("https://jinqiu.wang".to_string());
  contact.email = Some("jqiue@foxmail.com".to_string());
  api.info.contact = Some(contact);
}
