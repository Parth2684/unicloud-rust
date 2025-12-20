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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DriveListResponse {
    files: Vec<DriveFile>,
    next_page_token: Option<String>,
}

pub async fn google_search_folder(
    folder_id: &str,
    token: &str,
) -> Result<Vec<DriveFile>, reqwest::Error> {
    let client = Client::new();
    let mut all_files: Vec<DriveFile> = Vec::new();
    let mut next_page_token: Option<String> = None;

    loop {
        let mut url = format!(
            "https://www.googleapis.com/drive/v3/files\
            ?q='{}' in parents and trashed=false\
            &fields=files(id,name,mimeType,parents,size,createdTime,modifiedTime),nextPageToken\
            &supportsAllDrives=true\
            &includeItemsFromAllDrives=true\
            &spaces=drive",
            folder_id
        );

        if let Some(ref token) = next_page_token {
            url.push_str(&format!("&pageToken={}", token));
        }

        let res = client
            .get(url)
            .bearer_auth(token)
            .send()
            .await?
            .json::<DriveListResponse>()
            .await?;

        all_files.extend(res.files);

        match res.next_page_token {
            Some(token) => next_page_token = Some(token),
            None => break,
        }
    }

    Ok(all_files)
}
