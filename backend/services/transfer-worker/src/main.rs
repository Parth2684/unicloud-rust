use std::{num::NonZeroUsize, sync::Arc, thread};

use common::{db_connect::init_db, export_envs::ENVS, redis_connection::init_redis};
use entities::job::{Column as JobColumn, Entity as JobEntity};
use redis::AsyncTypedCommands;
use tokio::{sync::Semaphore, task};

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

use crate::job::process_job;
mod job;
mod handlers;

#[tokio::main]
async fn main() {
    let (mut redis_conn, db) = tokio::join!(init_redis(), init_db());
    let multiple: usize;
    if ENVS.environment == "PRODUCTION" {
        multiple = 2
    } else {
        multiple = 1
    }
    let max_workers = thread::available_parallelism()
        .map(NonZeroUsize::get)
        .unwrap_or(4)
        * multiple;
    let semaphore = Arc::new(Semaphore::new(max_workers));

    loop {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let job_id = redis_conn
            .brpoplpush("copy:job", "proccessing", 1.0)
            .await
            .ok();
        if let Some(id) = job_id {
            match id {
                None => continue,
                Some(id) => {
                    let job = JobEntity::find().filter(JobColumn::Id.eq(id)).one(db).await;
                    match job {
                        Err(err) => {
                            eprintln!("Error getting Job: {:?}", err);
                            continue;
                        }
                        Ok(job_optional) => match job_optional {
                            None => eprintln!("Job not found"),
                            Some(job) => {
                                task::spawn(async move {
                                    let _permit = permit;
                                    process_job(job).await;
                                });
                            }
                        },
                    }
                }
            }
        }
    }
}
