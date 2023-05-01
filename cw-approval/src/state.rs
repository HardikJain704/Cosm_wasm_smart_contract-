use cosmwasm_std::{Storage, Addr};
use cosmwasm_storage::Bucket;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Serialize , Deserialize};

// use crate::msg::Announcement;

#[derive(Debug , Serialize , Deserialize , Clone , PartialEq , Eq, JsonSchema)]


pub struct LeaveRequest {
    pub employee: u32,
    pub reason: String,
    pub approved: String,
    pub feedback: String,
}

#[derive(Serialize , Deserialize)]
pub struct Announcement {
    admin: String,
    message: String,

}

#[derive(Serialize , Deserialize)]
pub struct State {
     announcements: Vec<Announcement>,
     leave_requests: Vec<LeaveRequest>,
    
     
}

// impl Store {
//     pub fn from_storage(storage: &mut dyn Storage) -> Self {
//         Self {
//             leave_balances: Bucket::new(b"leave_balance", storage),
//         }
//     }
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LeaveBalance {
    pub emp_id: String,
    pub balances: Vec<u32>,
}

pub struct UpdateLeaveBalance {
    pub emp_id : String,
    pub leave_balance: u32,

}

// #[derive(Serialize, Deserialize, Clone, Default)]
// pub struct Store {
//     pub leave_balances: Bucket<LeaveBalance>,

// }


pub const OWNER: Item<String> = Item::new("Owner");

pub const DATASTORE:Item<Addr> = Item::new("contract_storage");


pub const STATE : Item<Vec<LeaveRequest>> = Item::new("STATE");

