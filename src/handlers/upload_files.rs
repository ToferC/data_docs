use actix_web::{post, web};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use serde::{Serialize, Deserialize};
use docx::{Document, DocxFile, Paragraph};

#[post("/upload_files")]
async fn upload_files(mut payload: Multipart) -> impl Responder {

    // get each file
    while let Ok(Some(mut field)) = payload.try_next().await {
        // transform from docx to data_doc associated with user
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();

        let mut rdr = Document::from()
        let docx = DocxFile::from_reader(rdr).unwrap();

    }



    // Create new template?


    // Let user know

}