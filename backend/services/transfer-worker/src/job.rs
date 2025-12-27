use entities::{job::Model as JobModel, sea_orm_active_enums::TransferType};

use crate::handlers::{google_to_google::copy_google_to_google, mega_to_google::copy_mega_to_google};




pub async fn process_job(job: JobModel) {
    match job.transfer_type {
        TransferType::MegaToGoogle => {
            copy_mega_to_google(job).await
        },
        TransferType::GoogleToGoogle => {
            copy_google_to_google(job).await;
        }
    }
}