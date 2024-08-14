#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User{
    username:String,
    id:u64,
    officialemail:String,
    phonenumber:String,
    created_at:u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Business{
    id:u64,
    nameofbusiness:String,
    ownerofbusiness:String,
    productselling:String,
    priceofproducts:u64,
    businessemail:String,
    businessphonenumber:String,
    loactionofbusiness:String,
    created_at:u64
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EnquireAboutAbusiness{
    id:u64,
    nameofbusiness:String,
    by:String,
    enquire:String,
    useremail:String,
    created_at:u64,
}
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CommentAboutOurServices{
    id:u64,
    by:String,
    comment:String,
    useremail:String,
    created_at:u64
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RaiseAquestionAboutOurServices{
    id:u64,
    question:String,
    useremail:String,
    created_at:u64
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}
impl Storable for Business {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Business {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}
impl Storable for EnquireAboutAbusiness {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for EnquireAboutAbusiness {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}
impl Storable for CommentAboutOurServices {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for CommentAboutOurServices {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}
impl Storable for RaiseAquestionAboutOurServices {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for RaiseAquestionAboutOurServices {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}
// thread
thread_local! {
    static MEMORY_MANAGER:RefCell<MemoryManager<DefaultMemoryImpl>>=RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    static ID_COUNTER:RefCell<IdCell>=RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),0).expect("Cannot create a counter")
    );
    static USERS_STORAGE:RefCell<StableBTreeMap<u64,User,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
    static BUSINESS_STORAGE:RefCell<StableBTreeMap<u64,Business,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
     
    static ENQURES_STORAGE:RefCell<StableBTreeMap<u64,EnquireAboutAbusiness,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
    static COMMENTS_STORAGE:RefCell<StableBTreeMap<u64,CommentAboutOurServices,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
    static QUESTIONS_STORAGE:RefCell<StableBTreeMap<u64,RaiseAquestionAboutOurServices,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));


}

#[derive(candid::CandidType,Clone,Serialize,Deserialize,Default)]
struct UserPayload{
    username:String,
    phonenumber:String,
    officialemail:String,   
}
#[derive(candid::CandidType,Clone,Serialize,Deserialize,Default)]
struct BusinessPayload{

    nameofbusiness:String,
    ownerofbusiness:String,
    productselling:String,
    priceofproducts:u64,
    loactionofbusiness:String,
    businessphonenumber:String,
    businessemail:String,
   
}
#[derive(candid::CandidType,Clone,Serialize,Deserialize,Default)]
struct SearchABusinessPayload{
    businessid:u64,
}
#[derive(candid::CandidType,Clone,Serialize,Deserialize,Default)]

struct UpdateBusinessPayload{

    nameofbusiness:String,
    ownerofbusiness:String,
    productselling:String,
    priceofproducts:u64,
    loactionofbusiness:String,
    businessphonenumber:String,
    businessemail:String,
    businessid:u64
   
}

#[derive(candid::CandidType,Clone,Serialize,Deserialize,Default)]

struct BusinessEnquirePayload{

    nameofbusiness:String,
    by:String,
    enquire:String,
    useremail:String,
   
}
#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct CommentPayload{
    by:String,
    comment:String,
    useremail:String,
}
#[derive(candid::CandidType,Clone,Serialize,Deserialize,Default)]

struct QuestionPayload{
    question:String,
    useremail:String,
}
#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct DeletePayload{
    bizid:u64,
    username:String
}
#[ic_cdk::update]
fn register_user(payload: UserPayload) -> Result<User, String> {
    // Validate the payload to ensure that the required fields are present
  
    if payload.username.is_empty()
        ||payload.officialemail.is_empty()
        ||payload.phonenumber.is_empty()
    
    {
        return Err("All fields are required".to_string());
    }

    // Validate the payload to ensure that the email format is correct
    if !payload.officialemail.contains('@') {
        return Err("enter correct email format".to_string());
    }

    // Ensure email address uniqueness and ownername and also transport name
    let email_exists:bool = USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, val)| val.officialemail == payload.officialemail)
    });
    if email_exists {
        return Err("Email already exists".to_string());
    }

   let username_exists:bool=USERS_STORAGE.with(|storage| {
    storage
        .borrow()
        .iter()
        .any(|(_,val)| val.username == payload.username)
});
if username_exists {
    return Err("The username already exists".to_string());
}
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");
   
    let newuser = User {
        username:payload.username,
        id,
        officialemail:payload.officialemail,
        phonenumber:payload.phonenumber,
        created_at:time()
       
    };

    USERS_STORAGE.with(|storage| storage.borrow_mut().insert(id, newuser.clone()));

    Ok(newuser)
}


//users register their business and products they are actaually sellings
#[ic_cdk::update]
fn register_business(payload: BusinessPayload) -> Result<Business, String> {
    // Validate the payload to /ensure that the required fields are present
  
    if payload.nameofbusiness.is_empty()
        ||payload.ownerofbusiness.is_empty()
        ||payload.businessphonenumber.is_empty()
        ||payload.businessemail.is_empty()
        ||payload.loactionofbusiness.is_empty()
        ||payload.productselling.is_empty()
    
    {
        return Err("All fields are required".to_string());
    }

    // Validate the payload to ensure that the email format is correct
    if !payload.businessemail.contains('@') {
        return Err("enter correct email format".to_string());
    }
//ensure that user is already registered
   let username_exists:bool=USERS_STORAGE.with(|storage| {
    storage
        .borrow()
        .iter()
        .any(|(_,val)| val.username == payload.ownerofbusiness)
});
if !username_exists {
    return Err("You must be registered in order to continue".to_string());
}
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");
   
    let new_business = Business {
    id,
    nameofbusiness:payload.nameofbusiness,
    ownerofbusiness:payload.ownerofbusiness,
    productselling:payload.productselling,
    priceofproducts:payload.priceofproducts,
    businessemail:payload.businessemail,
    businessphonenumber:payload.businessphonenumber,
    loactionofbusiness:payload.loactionofbusiness,
    created_at:time()
       
    };

    BUSINESS_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_business.clone()));

    Ok(new_business)
}

//function to get all business
#[ic_cdk::query]
fn get_all_business() -> Result<Vec<Business>, String> {

    let business = BUSINESS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, vals)| vals.clone())
            .collect::<Vec<Business>>()
    });

    if  business.is_empty() {
        return Err("Currently no registered business are  available.".to_string());
    }

    else {
        Ok(business)
    }

}

//search fo a single business by id

#[ic_cdk::query]
fn search_for_a_business_by_id(payload:SearchABusinessPayload)->Result<Business,String>{
    let business = BUSINESS_STORAGE.with(|storage| storage.borrow().get(&payload.businessid));
    match business {
        Some(business) => Ok(business),
        None => Err("no business with such id that exists.".to_string()),
    }
    
}

//update business datails by the owner
#[ic_cdk::update]
fn update_business(payload:UpdateBusinessPayload)->Result<Business,String>{
    if payload.nameofbusiness.is_empty()
    ||payload.ownerofbusiness.is_empty()
    ||payload.businessphonenumber.is_empty()
    ||payload.businessemail.is_empty()
    ||payload.loactionofbusiness.is_empty()
    ||payload.productselling.is_empty()

{
    return Err("All fields are required".to_string());
}
     // Validate the payload to ensure that the email format is correct
     if !payload.businessemail.contains('@') {
        return Err("Invalid email format".to_string());
    }
    
  
    
match BUSINESS_STORAGE.with(|service|service.borrow().get(&payload.businessid)){
    Some(mut biz)=>{    
                        biz.nameofbusiness=payload.nameofbusiness;
                        biz.productselling=payload.productselling;
                        biz.priceofproducts=payload.priceofproducts;
                        biz.loactionofbusiness=payload.loactionofbusiness;
                        biz.businessphonenumber=payload.businessphonenumber;
                        biz.businessemail=payload.businessemail;
                        do_insert(&biz);
                        Ok(biz)
                        
    }
    None=>Err("could not update business details".to_string()),
}

}
//users delete their business from biz marketer
//owner delete business from online marketers site
#[ic_cdk::update]
  fn remove_your_business_from_onlinemarketers(payload:DeletePayload)->Result<String,String>{
 //verify its the owner of business is deleteing it
 let checkuser_exists:bool=USERS_STORAGE.with(|storage| {
    storage
        .borrow()
        .iter()
        .any(|(_,val)| val.username == payload.username)
});
if !checkuser_exists {
    return Err("you must be registered".to_string());
}
    match BUSINESS_STORAGE.with(|storage|storage.borrow_mut().remove(&payload.bizid)){
        Some(_val)=>Ok("you have successfully removed you business from biz marketers".to_string()),
        None=>Err("coulde not remove business form biz marketers".to_string(),)
    }
  }

//enquire about a businss
#[ic_cdk::update]
fn users_enquire_about_a_business(payload:BusinessEnquirePayload)->Result<EnquireAboutAbusiness,String>{


      // Validate the payload to ensure that the required fields are present
      if payload.by.is_empty()
      || payload.enquire.is_empty()
      ||payload.useremail.is_empty()
      ||payload.nameofbusiness.is_empty()
       {
          return Err("All fields are required".to_string());
       }
       // Validate the payload to ensure that the email format is correct
    if !payload.useremail.contains('@') {
        return Err("enter correct email format".to_string());
    }
//ensure that the business actually exists
let business_exists:bool=BUSINESS_STORAGE.with(|storage| {
    storage
        .borrow()
        .iter()
        .any(|(_,val)| val.nameofbusiness == payload.nameofbusiness)
});
if !business_exists {
    return Err("businss does not exist".to_string());
}
    let id = ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("Cannot increment ID counter");
    let new_business_enquire=EnquireAboutAbusiness{
    id,
    nameofbusiness:payload.nameofbusiness,
    by:payload.by,
    enquire:payload.enquire,
    useremail:payload.useremail,
    created_at:time()
     };
ENQURES_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_business_enquire.clone()));

return Ok(new_business_enquire);
}

//user comments
#[ic_cdk::update]
  fn users_commets_about_our_services(payload:CommentPayload)->Result<CommentAboutOurServices,String>{
    if payload.by.is_empty()
    ||payload.useremail.is_empty()
    || payload.comment.is_empty()
     {
        return Err("some fields are missing".to_string());
     }
     //check if user is registered to nsure aonly users who are registered can comment
    let checkuser_exists:bool=USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_,val)| val.username == payload.by)
    });
    if !checkuser_exists {
        return Err("You are not registered".to_string());
    }
     let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");
     let new_comment=CommentAboutOurServices{
        id,
        by:payload.by,
        comment:payload.comment,
        useremail:payload.useremail,
        created_at:time()
     };
     COMMENTS_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_comment.clone()));
     return Ok(new_comment);
  }
//get all comments
#[ic_cdk::query]
fn get_all_comments() -> Result<Vec<CommentAboutOurServices>, String> {

    let comments = COMMENTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, trans)| trans.clone())
            .collect::<Vec<CommentAboutOurServices>>()
    });

    if  comments.is_empty() {
        return Err("Currently no comments available.".to_string());
    }

    else {
        Ok(comments)
    }

}
//users ask questions about our services
#[ic_cdk::update]
  fn users_ask_questions_about_our_services(payload:QuestionPayload)->Result<RaiseAquestionAboutOurServices,String>{
    if payload.question.is_empty()
    ||payload.useremail.is_empty()
     {
        return Err("some fields are missing".to_string());
     }
     
     let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");
     let new_question=RaiseAquestionAboutOurServices{
        id,
        question:payload.question,
        useremail:payload.useremail,
        created_at:time()
     };
     QUESTIONS_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_question.clone()));
     return Ok(new_question);
  }

  //function to retrives all questions asked
  #[ic_cdk::query]
fn get_all_questions() -> Result<Vec<RaiseAquestionAboutOurServices>, String> {

    let questions = QUESTIONS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, trans)| trans.clone())
            .collect::<Vec<RaiseAquestionAboutOurServices>>()
    });

    if  questions.is_empty() {
        return Err("Currently no comments available.".to_string());
    }

    else {
        Ok(questions)
    }

}
//helper unction for updates
fn do_insert(biz:&Business){
    BUSINESS_STORAGE.with(|service|service.borrow_mut().insert(biz.id,biz.clone()));
}
ic_cdk::export_candid!();