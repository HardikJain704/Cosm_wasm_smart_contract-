use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};


use crate::state::{LeaveRequest, LeaveBalance};


#[cw_serde]

pub struct InstantiateMsg {
  pub employee: u32,
  pub reason: String,
  pub approved: String,
  pub feedback: String,
  pub owner:String,
  pub contract_address : Addr

}


#[cw_serde]
pub enum Msg {
    RequestLeave { emp_id: u32 , reason: String , leave_days: u32},
    ApproveLeave { employee: u32, feedback: String },
    RejectLeave { employee: u32, feedback: String },
    PostAnnouncement { announcement: String },
  
}


#[cw_serde]

#[derive(QueryResponses)]

pub enum QueryMsg {
     #[returns(Vec<LeaveRequest>)]
    GetAnnouncements{ feedback : String , owner: String},

    // GetLeaveBalance { emp_id :  String},
    
}
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub enum StoreMsg {

//     UpdateLeaveBalance { emp_id: String, leave_balance: u32 },
    

// }

// impl Store {
//   pub fn update_leave_balance(&mut self, emp_id: &str, new_balance: u32) -> StdResult<()> {
//       let mut balances = self
//           .leave_balances
//           .get(emp_id.as_bytes())
//           .unwrap_or_else(|| LeaveBalance::new(emp_id))
//           .to_vec();
//       balances.push(new_balance);
//       self.leave_balances
//           .set(emp_id.as_bytes(), &balances)
//           .map_err(|e| e.into())
//   }
// }

// #[derive(Serialize, Deserialize , Clone , Debug , PartialEq)]
// pub enum HandleMsg {
//     CreateLeaveRequest { reason: String },
//     ApproveLeaveRequest { requester: String, feedback: String },
//     RejectLeave {requester: String , feedback: String},
//     PostAnnouncement { message: String },
// }


