use anyhow::anyhow;
use argon2::password_hash::{Ident, SaltString};
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use std::env;
use std::fmt;
use std::fmt::Formatter;
use url_wrap_kernel::model::wrap::PHCString;

pub struct HashedPassword(String);

impl HashedPassword {
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }

    pub fn verify(&self, password: &str) -> anyhow::Result<()> {
        verify_password(&self.to_string(), password)
    }
}

impl fmt::Display for HashedPassword {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for HashedPassword {
    type Error = anyhow::Error;

    fn try_from(password: String) -> Result<Self, Self::Error> {
        let hashed = hash_password(&password)?;
        Ok(HashedPassword(hashed))
    }
}

impl From<HashedPassword> for PHCString {
    fn from(hp: HashedPassword) -> Self {
        Self(hp.to_string())
    }
}

struct HashingParameter {
    salt: String,
    variant: String,
    version: u32,
    time_cost: u32,
    memory_cost: u32,
    parallelism_cost: u32,
}

impl HashingParameter {
    fn new(
        salt: String,
        variant: String,
        version: u32,
        time_cost: u32,
        memory_cost: u32,
        parallelism_cost: u32,
    ) -> Self {
        Self {
            salt,
            variant,
            version,
            time_cost,
            memory_cost,
            parallelism_cost,
        }
    }
}

fn hash_password(password: &str) -> anyhow::Result<String> {
    let bin_password = password.as_bytes();
    let encryption_data = init_hashing_parameter();

    let salt_string = SaltString::new(&encryption_data.salt).map_err(|e| anyhow!(e))?;

    // Argon2 with customized params
    let ident = Ident::try_from(encryption_data.variant.as_str()).map_err(|e| anyhow!(e))?;
    let algorithm = Algorithm::try_from(ident).map_err(|e| anyhow!(e))?;
    let version = Version::try_from(encryption_data.version).map_err(|e| anyhow!(e))?;

    let params = Params::new(
        encryption_data.memory_cost,
        encryption_data.time_cost,
        encryption_data.parallelism_cost,
        None,
    )
    .map_err(|e| anyhow!(e))?;
    let argon2 = Argon2::new(algorithm, version, params);

    Ok(argon2
        .hash_password(bin_password, &salt_string)
        .map_err(|e| anyhow!(e))?
        .to_string())
}

fn verify_password(hashed_password: &str, password: &str) -> anyhow::Result<()> {
    let bin_password = password.as_bytes();
    let password_hash = PasswordHash::new(hashed_password).map_err(|e| anyhow!(e))?;
    match Argon2::default().verify_password(bin_password, &password_hash) {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!(err)),
    }
}

fn init_hashing_parameter() -> HashingParameter {
    let salt = env::var_os("ARGON2_PHC_SALT")
        .expect("ARGON2_PHC_SALT is undefined.")
        .into_string()
        .expect("ARGON2_PHC_SALT is invalid value.");
    let variant = env::var_os("ARGON2_PHC_VARIANT")
        .expect("ARGON2_PHC_VARIANT is undefined.")
        .into_string()
        .expect("ARGON2_PHC_VARIANT is invalid value.");
    let version = env::var_os("ARGON2_PHC_VERSION")
        .expect("ARGON2_PHC_VERSION is undefined.")
        .into_string()
        .expect("ARGON2_PHC_VERSION is invalid value.")
        .parse::<u32>()
        .expect("ARGON2_PHC_VERSION is invalid value.");
    let time_cost = env::var_os("ARGON2_PHC_TIME_COST")
        .expect("ARGON2_PHC_TIME_COST is undefined.")
        .into_string()
        .expect("ARGON2_PHC_TIME_COST is invalid value.")
        .parse::<u32>()
        .expect("ARGON2_PHC_TIME_COST is invalid value.");
    let memory_cost = env::var_os("ARGON2_PHC_MEMORY_COST")
        .expect("ARGON2_PHC_MEMORY_COST is undefined.")
        .into_string()
        .expect("ARGON2_PHC_MEMORY_COST is invalid value.")
        .parse::<u32>()
        .expect("ARGON2_PHC_MEMORY_COST is invalid value.");
    let parallelism_cost = env::var_os("ARGON2_PHC_PARALLELISM_COST")
        .expect("ARGON2_PHC_PARALLELISM_COST is undefined.")
        .into_string()
        .expect("ARGON2_PHC_PARALLELISM_COST is invalid value.")
        .parse::<u32>()
        .expect("ARGON2_PHC_PARALLELISM_COST is invalid value.");

    HashingParameter::new(
        salt,
        variant,
        version,
        time_cost,
        memory_cost,
        parallelism_cost,
    )
}
