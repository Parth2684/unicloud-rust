use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    slice::from_mut,
    sync::{Arc, mpsc},
    thread, vec,
};

use axum::{
    Extension, Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use common::{db_connect::init_db, encrypt::decrypt, jwt_config::Claims};
use entities::{
    cloud_account::{Column as CloudAccountColumn, Entity as CloudAccountEntity},
    sea_orm_active_enums::Provider,
};
use futures::future::join_all;
use reqwest::Client;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;

use crate::utils::app_errors::AppError;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct DriveFile {
    id: String,
    name: String,
    mimeType: String,
    #[serde(default)]
    parents: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileListResponse {
    files: Vec<DriveFile>,
    #[serde(default)]
    nextPageToken: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub mimeType: String,
    pub children: Vec<Node>,
}

#[derive(Clone)]
struct FileStructureName {
    drive_name: String,
    files: Option<Vec<DriveFile>>,
    error: Option<String>,
}

#[derive(Serialize)]
struct Drive {
    drive_name: String,
    files: Vec<Node>,
}

async fn fetch_all_files(access_token: &str) -> reqwest::Result<Vec<DriveFile>> {
    let client = Client::new();
    let mut all_files: Vec<DriveFile> = Vec::new();
    let mut page_token: Option<String> = None;
    loop {
        let mut url = String::from(
            "https://www.googleapis.com/drive/v3/files\
                    ?spaces=drive\
                    &corpora=user\
                    &includeItemsFromAllDrives=true\
                    &supportsAllDrives=true\
                    &q=trashed=false\
                    &fields=files(id,name,mimeType,parents),nextPageToken",
        );
        if let Some(token) = page_token.clone() {
            url.push_str(&format!("&pageToken={}", token));
        }

        let res = client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<FileListResponse>()
            .await?;

        all_files.extend(res.files);

        match res.nextPageToken {
            Some(token) => page_token = Some(token),
            None => break,
        }
    }
    dbg!(&all_files);
    Ok(all_files)
}

#[derive(Clone)]
struct Map {
    parents: Vec<String>,
    file: Node,
}

fn find_node_mut<'a>(nodes: &'a mut [Node], target_id: &str) -> Option<&'a mut Node> {
    for node in nodes {
        if node.id == target_id {
            return Some(node);
        }
        if let Some(found) = find_node_mut(&mut node.children, target_id) {
            return Some(found);
        }
    }
    None
}

fn build_tree(files: &mut Vec<DriveFile>) -> Vec<Node> {
    let maps: Rc<RefCell<Vec<Map>>> = Rc::new(RefCell::new(Vec::new()));
    let roots: Rc<RefCell<Vec<Node>>> = Rc::new(RefCell::new(Vec::new()));

    for f in files {
        let get_roots = Rc::clone(&roots);
        if f.parents.is_empty() {
            get_roots.borrow_mut().push(Node {
                id: f.id.clone(),
                name: f.name.clone(),
                mimeType: f.mimeType.clone(),
                children: vec![],
            });
        } else {
            maps.borrow_mut().push(Map {
                parents: f.parents.clone(),
                file: Node {
                    id: f.id.clone(),
                    name: f.name.clone(),
                    mimeType: f.mimeType.clone(),
                    children: vec![],
                },
            });
        }
    }

    loop {
        let fill_roots = Rc::clone(&roots);
        let loop_maps = Rc::clone(&maps);
        let mut pushed_ids: Vec<String> =
            fill_roots.borrow().iter().map(|f| f.id.clone()).collect();

        for f in loop_maps.borrow_mut().iter_mut() {
            if pushed_ids.contains(&f.file.id) {
                continue;
            }
            for parent in f.parents.clone() {
                let mut to_find = fill_roots.borrow_mut();
                let parent_node = find_node_mut(&mut to_find, &parent);
                match parent_node {
                    None => (),
                    Some(pn) => {
                        pn.children.push(f.file.clone());
                        pushed_ids.push(f.file.id.clone());
                    }
                }
            }
        }
    }
    roots.into_inner()
}

pub async fn google_drive_file_structure(
    Extension(claims): Extension<Claims>,
) -> Result<Response, AppError> {
    let db = init_db().await;
    let cloud_accounts = CloudAccountEntity::find()
        .filter(CloudAccountColumn::UserId.eq(claims.id))
        .filter(CloudAccountColumn::Provider.eq(Provider::Google))
        .all(db)
        .await;

    match cloud_accounts {
        Ok(accs) => {
            if accs.len() == 0 {
                return Ok((
                    StatusCode::OK,
                    Json(json!({
                        "message": "No Accounts Found",
                        "cloud_accounts": []
                    })),
                )
                    .into_response());
            } else {
                let mut acc_token: HashMap<String, String> = HashMap::new();
                accs.iter().for_each(|acc| {
                    let decrypt_token = decrypt(&acc.access_token).ok();
                    if let Some(token) = decrypt_token {
                        acc_token.insert(acc.email.clone(), token);
                    };
                });

                let folder_structures: Arc<Mutex<Vec<FileStructureName>>> =
                    Arc::new(Mutex::new(Vec::new()));

                let mut tasks = vec![];

                for token in acc_token {
                    let folder_structure_clone = Arc::clone(&folder_structures);
                    tasks.push(tokio::spawn(async move {
                        let mut folders = folder_structure_clone.lock().await;
                        match fetch_all_files(&token.1).await {
                            Ok(files) => folders.push(FileStructureName {
                                drive_name: token.0,
                                files: Some(files),
                                error: None,
                            }),
                            Err(err) => folders.push(FileStructureName {
                                drive_name: token.0,
                                files: None,
                                error: Some(err.to_string()),
                            }),
                        };
                    }));
                }

                for task in join_all(tasks).await {
                    match task {
                        Ok(t) => t,
                        Err(err) => {
                            eprintln!("Join error: {err:?}");
                            return Err(AppError::Internal(Some(String::from(
                                "Error during joining fetch tasks",
                            ))));
                        }
                    }
                }

                let folder_structure_clone_2 = Arc::clone(&folder_structures);

                {
                    let folder_lock = folder_structure_clone_2.lock().await;
                    let mut no_error_drives: HashMap<String, Vec<DriveFile>> = HashMap::new();
                    let mut error_drives: HashMap<String, String> = HashMap::new();
                    folder_lock.iter().for_each(|fsn| {
                        if let Some(drives) = &fsn.files {
                            no_error_drives.insert(fsn.drive_name.clone(), drives.clone());
                        }
                        if let Some(error) = &fsn.error {
                            error_drives.insert(fsn.drive_name.clone(), error.clone());
                        }
                    });
                    let (trs, rec) = mpsc::channel();
                    for drive in no_error_drives {
                        let trs_clone = trs.clone();
                        thread::spawn(move || {
                            let tree = build_tree(&drive.1);
                            let result = Drive {
                                drive_name: drive.0.clone(),
                                files: tree,
                            };
                            let _ = trs_clone.send(result);
                        });
                    }

                    drop(trs);

                    let mut google_drives = Vec::new();
                    while let Ok(drive) = rec.recv() {
                        google_drives.push(drive);
                    }

                    return Ok((
                        StatusCode::OK,
                        Json(json!({
                            "google_drive": google_drives,
                            "error_drives": error_drives,
                            "message": "Fetching Successfull"
                        })),
                    )
                        .into_response());
                }
            }
        }
        Err(_) => {
            return Err(AppError::Internal(Some(String::from(
                "Error connecting to db",
            ))));
        }
    };
}
