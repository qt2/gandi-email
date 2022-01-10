use anyhow::Result;
use reqwest::{header::AUTHORIZATION, Response};
use serde::{Deserialize, Serialize};

/// Mailbox struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct Mailbox {
    pub address: String,
    pub id: String,
    pub alias_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct MailboxDetail {
    aliases: Vec<String>,
}

/// Instance of API.
#[derive(Debug)]
pub struct GandiEmailAPI {
    api_key: String,
}

impl GandiEmailAPI {
    /// Create a new API instance.
    pub fn new(api_key: String) -> Self {
        GandiEmailAPI { api_key }
    }

    async fn get(&self, url: &str) -> Result<Response> {
        Ok(reqwest::Client::new()
            .get(format!("https://api.gandi.net/v5{}", url))
            .header(AUTHORIZATION, format!("Apikey {}", self.api_key))
            .send()
            .await?)
    }

    async fn patch(&self, url: &str, data: &impl Serialize) -> Result<Response> {
        Ok(reqwest::Client::new()
            .patch(format!("https://api.gandi.net/v5{}", url))
            .header(AUTHORIZATION, format!("Apikey {}", self.api_key))
            .json(data)
            .send()
            .await?)
    }

    /// Get all domains.
    pub async fn domains(&self) -> Result<Vec<String>> {
        #[derive(Debug, Serialize, Deserialize)]
        struct Domain {
            fqdn: String,
        }

        let res = self
            .get("/domain/domains")
            .await?
            .json::<Vec<Domain>>()
            .await?;
        let res = res.iter().map(|r| r.fqdn.clone()).collect();
        Ok(res)
    }

    /// Get all mailboxes.
    pub async fn mailboxes(&self, domain: &str) -> Result<Vec<Mailbox>> {
        let res = self
            .get(&format!("/email/mailboxes/{}", domain))
            .await?
            .json::<Vec<Mailbox>>()
            .await?;
        Ok(res)
    }

    /// Get all mailboxes.
    pub async fn aliases(&self, domain: &str, mailbox_id: &str) -> Result<Vec<String>> {
        let res = self
            .get(&format!("/email/mailboxes/{}/{}", domain, mailbox_id))
            .await?
            .json::<MailboxDetail>()
            .await?;
        Ok(res.aliases)
    }

    /// Create an alias
    pub async fn create_alias(&self, domain: &str, mailbox_id: &str, alias: &str) -> Result<()> {
        let mut aliases = self.aliases(domain, mailbox_id).await?;
        aliases.push(alias.to_string());
        let _res = self
            .patch(
                &format!("/email/mailboxes/{}/{}", domain, mailbox_id),
                &MailboxDetail { aliases },
            )
            .await?;
        Ok(())
    }

    /// Delete an alias
    pub async fn delete_alias(&self, domain: &str, mailbox_id: &str, alias: &str) -> Result<()> {
        let mut aliases = self.aliases(domain, mailbox_id).await?;
        aliases.retain(|e| e != alias);
        let _res = self
            .patch(
                &format!("/email/mailboxes/{}/{}", domain, mailbox_id),
                &MailboxDetail { aliases },
            )
            .await?;
        Ok(())
    }
}
