
#[derive(Debug, Clone)]
pub struct SerializedCertificate {
    pub version: i64,
    pub full: String,
    pub verify_only: String,
}

pub struct Certificates {
    pub version: i64,
    pub list: Vec<String>
}

pub struct GetListCmd {
    pub version: i64,
    pub verify_only: bool,
}

#[derive(Clone)]
pub struct RegisterCmd {
    pub signature_algorithm: String,
    pub crypto_algorithm: String,
    pub certificate_propagation_delay_seconds: i64,
    pub dat_issuance_duration_seconds: i64,
    pub dat_ttl_seconds: i64,
}

impl Certificates {
    pub fn size(&self) -> usize {
        self.list.len()
    }

    pub fn export(&self, prefix_version: bool) -> String {
        let mut result = String::new();

        if prefix_version {
            result.push_str(self.version.to_string().as_str());
            if !&self.list.is_empty() {
                result.push('\n');
            }
        }

        result.push_str(&self.list.join("\n"));

        result
    }
}
