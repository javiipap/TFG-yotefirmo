use openssl::pkcs12::Pkcs12;
use openssl::pkey::{PKey, Public};
use openssl::{error::ErrorStack, pkcs12::ParsedPkcs12_2};
use serde::{Deserialize, Serialize};

use openssl::x509::X509;

use crate::error::CryptError;

pub struct Certificate {
    pub certificate: Pkcs12,
    unlocked_certificate: Option<ParsedPkcs12_2>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CertificateInfo {
    pub iat: String,
    pub exp: String,
    pub issuer: String,
    pub subj: String,
}

impl Certificate {
    pub fn new(certificate_value: &Vec<u8>) -> Self {
        Certificate {
            certificate: Pkcs12::from_der(&certificate_value).unwrap(),
            unlocked_certificate: None,
        }
    }

    pub fn unlock(&mut self, passphrase: String) -> Result<(), CryptError> {
        self.unlocked_certificate = Some(self.certificate.parse2(&passphrase)?);

        Ok(())
    }

    pub fn passphrase_required(&self) -> Result<bool, ErrorStack> {
        match self.certificate.parse2("") {
            Ok(_) => Ok(false),
            Err(e) => {
                if e.errors()
                    .iter()
                    .any(|e| e.reason() == Some("mac verify failure"))
                {
                    Ok(true)
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn extract_private_key(&self) -> Result<Vec<u8>, CryptError> {
        let parsed_cert = match &self.unlocked_certificate {
            Some(val) => val,
            None => return Err(CryptError::UnlockRequired),
        };

        let private_key = parsed_cert
            .pkey
            .as_ref()
            .expect("Certificate doesen't have a private key");

        Ok(private_key.private_key_to_der()?)
    }

    pub fn extract_public_key(&self) -> Result<Vec<u8>, CryptError> {
        let parsed_cert = match &self.unlocked_certificate {
            Some(val) => val,
            None => return Err(CryptError::UnlockRequired),
        };

        let public_key = match parsed_cert.cert.as_ref() {
            Some(val) => val.public_key()?,
            None => {
                return Err(CryptError::Unknown(
                    "No certificate could be extracted".to_string(),
                ))
            }
        };

        Ok(public_key.public_key_to_der()?)
    }

    fn get_email_address(subject: &openssl::x509::X509NameRef) -> Result<String, CryptError> {
        for entry in subject.entries() {
            let short_name = match entry.object().nid().short_name() {
                Ok(name) => name.to_string(),
                Err(_) => entry.object().to_string(),
            };

            if short_name == "emailAddress" {
                return Ok(entry.data().as_utf8().unwrap().to_string());
            }
        }

        Err(CryptError::default())
    }

    #[warn(non_snake_case)]
    pub fn ReadPublic(pem: &Vec<u8>) -> Result<(CertificateInfo, Vec<u8>), CryptError> {
        let certificate = X509::from_pem(pem)?;

        let public_key = certificate.public_key()?;

        Ok((
            CertificateInfo {
                iat: certificate.not_before().to_string(),
                exp: certificate.not_after().to_string(),
                issuer: certificate
                    .issuer_name()
                    .entries_by_nid(openssl::nid::Nid::COMMONNAME)
                    .next()
                    .unwrap()
                    .data()
                    .as_utf8()?
                    .to_string(),
                subj: Certificate::get_email_address(certificate.subject_name())?,
            },
            public_key.public_key_to_der()?,
        ))
    }

    #[warn(non_snake_case)]
    pub fn Info(unlocked_certificate: &ParsedPkcs12_2) -> Result<CertificateInfo, CryptError> {
        let cert = match &unlocked_certificate.cert {
            Some(val) => val,
            None => return Err(CryptError::UnlockRequired),
        };

        Ok(CertificateInfo {
            iat: cert.not_before().to_string(),
            exp: cert.not_after().to_string(),
            issuer: cert
                .issuer_name()
                .entries_by_nid(openssl::nid::Nid::COMMONNAME)
                .next()
                .unwrap()
                .data()
                .as_utf8()?
                .to_string(),
            subj: Certificate::get_email_address(cert.subject_name())?,
        })
    }

    pub fn info(&self) -> Result<CertificateInfo, CryptError> {
        if let Some(unlocked) = &self.unlocked_certificate {
            return Certificate::Info(&unlocked);
        }

        Err(CryptError::UnlockRequired)
    }

    pub fn to_der(&self) -> Vec<u8> {
        self.certificate.to_der().expect("Couldn't convert to DER")
    }

    pub fn to_pem(&self) -> Result<Vec<u8>, CryptError> {
        let parsed_cert = match &self.unlocked_certificate {
            Some(val) => val,
            None => return Err(CryptError::UnlockRequired),
        };

        Ok(parsed_cert.cert.as_ref().unwrap().to_pem().clone()?)
    }
}
