use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFile {
    pub id: String,
    pub name: String,
    pub mime_type: String,
    pub parents: Option<Vec<String>>,
    pub size: Option<String>,
    pub created_time: Option<String>,
    pub modified_time: Option<String>,
}

pub async fn google_search_folder(
    folder_id: &str,
    token: &str,
) -> Result<Vec<DriveFile>, reqwest::Error> {
    let client = Client::new();
    let url = format!(
        "https://www.googleapis.com/drive/v3/files
        ?q='{}' in parents and trashed=false
        &fields=files(id,name,mimeType,parents,size,createdTime,modifiedTime),nextPageToken
        &supportsAllDrives=true
        &includeItemsFromAllDrives=true
        &spaces=drive",
        folder_id
    );
    let res = client.get(url).bearer_auth(token).send().await;
    match res {
        Err(err) => {
            eprintln!("{err:?}");
            return Err(err);
        }
        Ok(data) => Ok(data.json::<Vec<DriveFile>>().await?),
    }
}
