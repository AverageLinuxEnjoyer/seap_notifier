use crate::advertisment::Advertisment;
use anyhow::Result;
use dotenv::dotenv;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport, Transport};
use std::env;

pub struct Mailer {
    smtp_mailer: SmtpTransport,
    name: String,
    email_address: String,
}

impl Mailer {
    pub fn new() -> Result<Self> {
        dotenv().ok();

        let name = env::var("NAME")?;
        let email_address = env::var("EMAIL_ADDRESS")?;
        let password = env::var("PASSWORD")?;

        let creds = Credentials::new(email_address.clone(), password);

        let relay = env::var("RELAY")?;

        Ok(Mailer {
            smtp_mailer: SmtpTransport::relay(&relay)?.credentials(creds).build(),
            name,
            email_address,
        })
    }

    pub async fn notify(&mut self, email_address: &str, ad: &Advertisment) -> Result<()> {
        let email = lettre::Message::builder()
            .from(format!("{} <{}>", self.name, self.email_address).parse()?)
            .to(format!("{} <{}>", "User", email_address).parse()?)
            .subject(format!("New ad: {}", ad.contract_object))
            .body(format!(
                "New ad: {}\n\n{}\n",
                ad.contract_object, ad.contract_description
            ))?;

        match self.smtp_mailer.send(&email) {
            Ok(res) => {
                println!("Email sent successfully!");
                println!("Response: {:?}", res);
            }
            Err(e) => println!("Could not send email: {:?}", e),
        };

        Ok(())
    }
}
