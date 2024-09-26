use crate::{idl::Idl, util::idl_type_to_rust_type};
use askama::Template;
use heck::ToSnekCase;
use heck::ToUpperCamelCase;
use sha2::{Digest, Sha256};

#[allow(dead_code)]
#[derive(Debug)]
pub struct AccountData {
    pub struct_name: String,
    pub module_name: String,
    pub discriminator: String,
    pub fields: Vec<FieldData>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FieldData {
    pub name: String,
    pub rust_type: String,
}

#[derive(Template)]
#[template(path = "accounts_struct.askama", escape = "none", ext = ".askama")]
pub struct AccountsStructTemplate<'a> {
    pub account: &'a AccountData,
}

#[derive(Template)]
#[template(path = "accounts_mod.askama", escape = "none", ext = ".askama")]
pub struct AccountsModTemplate<'a> {
    pub accounts: &'a Vec<AccountData>,
    pub decoder_name: String,
    pub program_struct_name: String,
}

pub fn process_accounts(idl: &Idl) -> Vec<AccountData> {
    let mut accounts_data = Vec::new();

    for account in &idl.accounts {
        let module_name = account.name.to_snek_case();
        let struct_name = account.name.to_upper_camel_case();
        // TODO: Check if it should be lower or upper camel case here, for instructions it's snake case
        let discriminator = compute_account_discriminator(&account.name);

        let mut fields = Vec::new();

        if let Some(ref fields_vec) = account.type_.fields {
            for field in fields_vec {
                let rust_type = idl_type_to_rust_type(&field.type_);
                fields.push(FieldData {
                    name: field.name.to_snek_case(),
                    rust_type,
                });
            }
        }

        accounts_data.push(AccountData {
            struct_name,
            module_name,
            discriminator,
            fields,
        });
    }

    accounts_data
}

fn compute_account_discriminator(account_name: &str) -> String {
    let mut hasher = Sha256::new();
    let discriminator_input = format!("account:{}", account_name);
    hasher.update(discriminator_input.as_bytes());
    let hash = hasher.finalize();
    let discriminator_bytes = &hash[..8];
    format!("0x{}", hex::encode(discriminator_bytes))
}
