use std::fs::File;
use std::io::Read;
use std::io::Write;

use bytes::BytesMut;
use futures::TryStreamExt;
use rusoto_s3::{GetObjectRequest, ListObjectsV2Request, PutObjectOutput, PutObjectRequest, S3, S3Client};
use rusoto_s3::ListObjectsV2Output;
use tokio;

use errors::CliConfigError;

use crate::errors;

const BUCKET: &'static str = "myawsaccountbucket";
const SSE_KMS_KEY_ID: &'static str = "xxxxxxxxxxxxxxxxxxxxxxxxx";
const SSE: &'static str = "aws:kms";
const CONTENT_TYPE: &'static str = "binary/octet-stream";


pub mod client {
    use rusoto_core::{HttpClient, Region};
    use rusoto_credential::ChainProvider;
    use rusoto_s3::S3Client;

    use errors::CliConfigError;

    use crate::errors;

    pub fn get_s3_client() -> Result<S3Client, CliConfigError> {
        let provider = ChainProvider::new();
        let http_client = HttpClient::new()?;

        Ok(S3Client::new_with(http_client, provider, Region::EuWest1))
    }
}

#[tokio::main]
pub async fn cmd_ls(s3_client: &S3Client, s3_path: &str) -> Result<ListObjectsV2Output, CliConfigError> {
    let list_obj_req = ListObjectsV2Request {
        bucket: BUCKET.to_owned(),
        prefix: Some(s3_path.to_owned()),
        ..Default::default()
    };
    let response = s3_client.list_objects_v2(list_obj_req).await;
    Ok(response?)
}

#[tokio::main]
pub async fn cmd_get<'a>(s3_client: &S3Client, s3_file: &'a str, save_to: &'a str) -> Result<&'a str, CliConfigError> {
    let get_obj_req = GetObjectRequest {
        bucket: BUCKET.to_owned(),
        key: s3_file.to_owned(),
        ..Default::default()
    };

    println!("Saving S3 file {} to {}", s3_file, save_to);
    let result = s3_client.get_object(get_obj_req).await;
    let body = result?.body;
    let stream = body.unwrap()
        .map_ok(|b| BytesMut::from(&b[..]))
        .try_concat()
        .await?;
    let mut file = File::create(save_to)?;
    file.write_all(&stream)?;
    Ok(save_to)
}

#[tokio::main]
pub async fn cmd_put(s3_client: &S3Client, conf_to_put: &str, s3_path: &str) -> Result<PutObjectOutput, CliConfigError> {
    let mut f = File::open(conf_to_put).unwrap();
    let mut contents: Vec<u8> = Vec::new();
    //read_to_end() needs use std::io::Read;
    f.read_to_end(&mut contents)?;

    let req = PutObjectRequest {
        bucket: BUCKET.to_owned(),
        key: s3_path.to_owned(),
        body: Some(contents.into()),
        ssekms_key_id: Some(SSE_KMS_KEY_ID.to_owned()),
        server_side_encryption: Some(SSE.to_owned()),
        content_type: Some(CONTENT_TYPE.to_owned()),
        ..Default::default()
    };

    println!("Uploading conf {} to S3 at {}", conf_to_put, s3_path);

    let result = s3_client.put_object(req).await;
    Ok(result?)
}

