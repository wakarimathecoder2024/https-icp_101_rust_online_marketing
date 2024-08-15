#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Type aliases for clarity
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Structs for different types of data
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    username: String,
    id: u64,
    officialemail: String,
    phonenumber: String,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Business {
    id: u64,
    nameofbusiness: String,
    ownerofbusiness: String,
    productselling: String,
    priceofproducts: u64,
    businessemail: String,
    businessphonenumber: String,
    loactionofbusiness: String,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EnquireAboutAbusiness {
    id: u64,
    nameofbusiness: String,
    by: String,
    enquire: String,
    useremail: String,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CommentAboutOurServices {
    id: u64,
    by: String,
    comment: String,
    useremail: String,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RaiseAquestionAboutOurServices {
    id: u64,
    question: String,
    useremail: String,
    created_at: u64,
}

// Implementing Storable and BoundedStorable for each struct
macro_rules! impl_storable {
    ($struct_name:ident) => {
        impl Storable for $struct_name {
            fn to_bytes(&self) -> Cow<[u8]> {
                Cow::Owned(Encode!(self).unwrap())
            }

            fn from_bytes(bytes: Cow<[u8]>) -> Self {
                Decode!(bytes.as_ref(), Self).unwrap()
            }
        }

        impl BoundedStorable for $struct_name {
            const MAX_SIZE: u32 = 512;
            const IS_FIXED_SIZE: bool = false;
        }
    };
}

impl_storable!(User);
impl_storable!(Business);
impl_storable!(EnquireAboutAbusiness);
impl_storable!(CommentAboutOurServices);
impl_storable!(RaiseAquestionAboutOurServices);

// Thread-local storage
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0).expect("Cannot create a counter")
    );
    static USERS_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
    static BUSINESS_STORAGE: RefCell<StableBTreeMap<u64, Business, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
    static ENQURES_STORAGE: RefCell<StableBTreeMap<u64, EnquireAboutAbusiness, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
    static COMMENTS_STORAGE: RefCell<StableBTreeMap<u64, CommentAboutOurServices, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
    static QUESTIONS_STORAGE: RefCell<StableBTreeMap<u64, RaiseAquestionAboutOurServices, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));
}

// Payload structs
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct UserPayload {
    username: String,
    phonenumber: String,
    officialemail: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct BusinessPayload {
    nameofbusiness: String,
    ownerofbusiness: String,
    productselling: String,
    priceofproducts: u64,
    loactionofbusiness: String,
    businessphonenumber: String,
    businessemail: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct SearchABusinessPayload {
    businessid: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct UpdateBusinessPayload {
    nameofbusiness: String,
    ownerofbusiness: String,
    productselling: String,
    priceofproducts: u64,
    loactionofbusiness: String,
    businessphonenumber: String,
    businessemail: String,
    businessid: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct BusinessEnquirePayload {
    nameofbusiness: String,
    by: String,
    enquire: String,
    useremail: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CommentPayload {
    by: String,
    comment: String,
    useremail: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct QuestionPayload {
    question: String,
    useremail: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct DeletePayload {
    bizid: u64,
    username: String,
}

// Validation Functions
fn validate_email(email: &str) -> Result<(), String> {
    if email.contains('@') {
        Ok(())
    } else {
        Err("Invalid email format".to_string())
    }
}

fn check_non_empty_fields(fields: &[(&str, &str)]) -> Result<(), String> {
    for (field_name, field_value) in fields {
        if field_value.is_empty() {
            return Err(format!("{} is required", field_name));
        }
    }
    Ok(())
}

fn generate_unique_id() -> u64 {
    ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1).expect("Failed to increment ID counter");
        current_value
    })
}

// API Functions
#[ic_cdk::update]
fn register_user(payload: UserPayload) -> Result<User, String> {
    check_non_empty_fields(&[
        ("Username", &payload.username),
        ("Official Email", &payload.officialemail),
        ("Phone Number", &payload.phonenumber),
    ])?;
    validate_email(&payload.officialemail)?;

    let email_exists = USERS_STORAGE.with(|storage| {
        storage.borrow().iter().any(|(_, val)| val.officialemail == payload.officialemail)
    });

    if email_exists {
        return Err("Email already exists".to_string());
    }

    let username_exists = USERS_STORAGE.with(|storage| {
        storage.borrow().iter().any(|(_, val)| val.username == payload.username)
    });

    if username_exists {
        return Err("The username already exists".to_string());
    }

    let id = generate_unique_id();
    let new_user = User {
        username: payload.username,
        id,
        officialemail: payload.officialemail,
        phonenumber: payload.phonenumber,
        created_at: time(),
    };

    USERS_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_user.clone()));

    Ok(new_user)
}

#[ic_cdk::update]
fn register_business(payload: BusinessPayload) -> Result<Business, String> {
    check_non_empty_fields(&[
        ("Business Name", &payload.nameofbusiness),
        ("Owner of Business", &payload.ownerofbusiness),
        ("Business Phone Number", &payload.businessphonenumber),
        ("Business Email", &payload.businessemail),
        ("Location of Business", &payload.loactionofbusiness),
        ("Product Selling", &payload.productselling),
    ])?;
    validate_email(&payload.businessemail)?;

    let username_exists = USERS_STORAGE.with(|storage| {
        storage.borrow().iter().any(|(_, val)| val.username == payload.ownerofbusiness)
    });

    if !username_exists {
        return Err("You must be registered to continue".to_string());
    }

    let id = generate_unique_id();
    let new_business = Business {
        id,
        nameofbusiness: payload.nameofbusiness,
        ownerofbusiness: payload.ownerofbusiness,
        productselling: payload.productselling,
        priceofproducts: payload.priceofproducts,
        businessemail: payload.businessemail,
        businessphonenumber: payload.businessphonenumber,
        loactionofbusiness: payload.loactionofbusiness,
        created_at: time(),
    };

    BUSINESS_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_business.clone()));

    Ok(new_business)
}

#[ic_cdk::query]
fn get_all_business() -> Result<Vec<Business>, String> {
    let businesses = BUSINESS_STORAGE.with(|storage| {
        storage.borrow().iter().map(|(_, val)| val.clone()).collect::<Vec<Business>>()
    });

    if businesses.is_empty() {
        Err("Currently no registered businesses are available.".to_string())
    } else {
        Ok(businesses)
    }
}

#[ic_cdk::query]
fn search_for_a_business_by_id(payload: SearchABusinessPayload) -> Result<Business, String> {
    BUSINESS_STORAGE.with(|storage| {
        storage.borrow().get(&payload.businessid).ok_or_else(|| "No business with such ID exists.".to_string())
    })
}

#[ic_cdk::update]
fn update_business(payload: UpdateBusinessPayload) -> Result<Business, String> {
    // Validate that all fields are present
    check_non_empty_fields(&[
        ("Business Name", &payload.nameofbusiness),
        ("Owner of Business", &payload.ownerofbusiness),
        ("Business Phone Number", &payload.businessphonenumber),
        ("Business Email", &payload.businessemail),
        ("Location of Business", &payload.loactionofbusiness),
        ("Product Selling", &payload.productselling),
    ])?;
    // Validate that the email is correctly formatted
    validate_email(&payload.businessemail)?;

    BUSINESS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut biz) = storage.remove(&payload.businessid) {
            biz.nameofbusiness = payload.nameofbusiness;
            biz.productselling = payload.productselling;
            biz.priceofproducts = payload.priceofproducts;
            biz.loactionofbusiness = payload.loactionofbusiness;
            biz.businessphonenumber = payload.businessphonenumber;
            biz.businessemail = payload.businessemail;
            storage.insert(payload.businessid, biz.clone());
            Ok(biz)
        } else {
            Err("Could not update business details: business not found".to_string())
        }
    })
}


#[ic_cdk::update]
fn remove_your_business_from_onlinemarketers(payload: DeletePayload) -> Result<String, String> {
    let user_exists = USERS_STORAGE.with(|storage| {
        storage.borrow().iter().any(|(_, val)| val.username == payload.username)
    });

    if !user_exists {
        return Err("You must be registered to perform this action.".to_string());
    }

    BUSINESS_STORAGE.with(|storage| {
        storage.borrow_mut().remove(&payload.bizid).map(|_| {
            "You have successfully removed your business from biz marketers".to_string()
        }).ok_or_else(|| "Could not remove business from biz marketers".to_string())
    })
}

#[ic_cdk::update]
fn users_enquire_about_a_business(payload: BusinessEnquirePayload) -> Result<EnquireAboutAbusiness, String> {
    check_non_empty_fields(&[
        ("By", &payload.by),
        ("Enquire", &payload.enquire),
        ("User Email", &payload.useremail),
        ("Business Name", &payload.nameofbusiness),
    ])?;
    validate_email(&payload.useremail)?;

    let business_exists = BUSINESS_STORAGE.with(|storage| {
        storage.borrow().iter().any(|(_, val)| val.nameofbusiness == payload.nameofbusiness)
    });

    if !business_exists {
        return Err("Business does not exist".to_string());
    }

    let id = generate_unique_id();
    let new_enquire = EnquireAboutAbusiness {
        id,
        nameofbusiness: payload.nameofbusiness,
        by: payload.by,
        enquire: payload.enquire,
        useremail: payload.useremail,
        created_at: time(),
    };

    ENQURES_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_enquire.clone()));

    Ok(new_enquire)
}

#[ic_cdk::update]
fn users_comments_about_our_services(payload: CommentPayload) -> Result<CommentAboutOurServices, String> {
    check_non_empty_fields(&[
        ("By", &payload.by),
        ("User Email", &payload.useremail),
        ("Comment", &payload.comment),
    ])?;
    validate_email(&payload.useremail)?;

    let user_exists = USERS_STORAGE.with(|storage| {
        storage.borrow().iter().any(|(_, val)| val.username == payload.by)
    });

    if !user_exists {
        return Err("You are not registered".to_string());
    }

    let id = generate_unique_id();
    let new_comment = CommentAboutOurServices {
        id,
        by: payload.by,
        comment: payload.comment,
        useremail: payload.useremail,
        created_at: time(),
    };

    COMMENTS_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_comment.clone()));

    Ok(new_comment)
}

#[ic_cdk::query]
fn get_all_comments() -> Result<Vec<CommentAboutOurServices>, String> {
    let comments = COMMENTS_STORAGE.with(|storage| {
        storage.borrow().iter().map(|(_, val)| val.clone()).collect::<Vec<CommentAboutOurServices>>()
    });

    if comments.is_empty() {
        Err("Currently no comments available.".to_string())
    } else {
        Ok(comments)
    }
}

#[ic_cdk::update]
fn users_ask_questions_about_our_services(payload: QuestionPayload) -> Result<RaiseAquestionAboutOurServices, String> {
    check_non_empty_fields(&[
        ("Question", &payload.question),
        ("User Email", &payload.useremail),
    ])?;
    validate_email(&payload.useremail)?;

    let id = generate_unique_id();
    let new_question = RaiseAquestionAboutOurServices {
        id,
        question: payload.question,
        useremail: payload.useremail,
        created_at: time(),
    };

    QUESTIONS_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_question.clone()));

    Ok(new_question)
}

#[ic_cdk::query]
fn get_all_questions() -> Result<Vec<RaiseAquestionAboutOurServices>, String> {
    let questions = QUESTIONS_STORAGE.with(|storage| {
        storage.borrow().iter().map(|(_, val)| val.clone()).collect::<Vec<RaiseAquestionAboutOurServices>>()
    });

    if questions.is_empty() {
        Err("Currently no questions available.".to_string())
    } else {
        Ok(questions)
    }
}

// Helper function for updating business details
fn do_insert(biz: &Business) {
    BUSINESS_STORAGE.with(|storage| storage.borrow_mut().insert(biz.id, biz.clone()));
}

ic_cdk::export_candid!();