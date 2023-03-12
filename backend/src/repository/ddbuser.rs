// use aws_sdk_dynamodb::Client;
// use aws_sdk_dynamodb::model::AttributeValue;
// use aws_config::Config;
// use log::error;

// use std::collections::HashMap;
// use crate::model::user::User;

// use super::DDBError;

// pub struct DDBUser {
//     client: Client,
//     table_name: String
// }


// fn required_item_value(key: &str, item: &HashMap<String, AttributeValue>) -> Result<String, DDBError> {
//     match item_value(key, item) {
//         Ok(Some(value)) => Ok(value),
//         Ok(None) => Err(DDBError),
//         Err(DDBError) => Err(DDBError)
//     }
// }

// fn item_value(key: &str, item: &HashMap<String, AttributeValue>) -> Result<Option<String>, DDBError> {
//     match item.get(key) {
//         Some(value) => match value.as_s() {
//             Ok(val) => Ok(Some(val.clone())),
//             Err(_) => Err(DDBError)
//         },
//         None => Ok(None)
//     }
// }

// fn item_to_task(item: &HashMap<String, AttributeValue>) -> Result<User, DDBError> {
//     let state: TaskState = match TaskState::from_str(required_item_value("state", item)?.as_str()) {
//         Ok(value) => value,
//         Err(_) => return Err(DDBError)
//     };

//     let result_file = item_value("result_file", item)?;

//     Ok(Task {
//         user_uuid: required_item_value("pK", item)?,
//         task_uuid: required_item_value("sK", item)?,
//         task_type: required_item_value("task_type", item)?,
//         state,
//         source_file: required_item_value("source_file", item)?,
//         result_file
//     })
// }

// impl DDBUser {
//     pub fn init(table_name: String, config: Config) -> DDBUser {
//         let client = Client::new(&config);
//         DDBUser {
//             table_name,
//             client
//         }
//     }

//     pub async fn put_user(&self, user: User) -> Result<(), DDBError> {
//         let mut request = self.client.put_item()
//             .table_name(&self.table_name)
//             .item("pK", AttributeValue::S(String::from(user.user_uuid)))
//             .item("sK", AttributeValue::S(String::from(user.user_uuid)))
//             .item("description", AttributeValue::S(String::from(user.description)));
        
//         match request.send().await {
//             Ok(_) => Ok(()),
//             Err(_) => Err(DDBError)
//         }
//     }

//     pub async fn get_user(&self, user_id: String) -> Option<User> {
//         let user_uuid = AttributeValue::S(user_id.clone());
        
//         let res = self.client
//             .query()
//             .table_name(&self.table_name)
//             .key_condition_expression("#pK = :user_id and #sK = :user_id")
//             .expression_attribute_names("#pK", "pK")
//             .expression_attribute_names("#sK", "sK")
//             .expression_attribute_values(":user_uuid", user_uuid)
//             .expression_attribute_values(":user_uuid", task_uuid)
//             .send()
//             .await;

//         return match res {
//             Ok(output) => {
//                 match output.items {
//                     Some(items) => {
//                         let item = &items.first()?;
//                         error!("{:?}", &item);
//                         match item_to_user(item) {
//                             Ok(user) => Some(user),
//                             Err(_) => None
//                         }
//                     },
//                     None => {
//                         None
//                     }
//                 }
//             },
//             Err(error) => {
//                 error!("{:?}", error);
//                 None
//             }
//         }
//     }
// }
