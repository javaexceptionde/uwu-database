use std::collections::HashMap;
use std::fmt::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

static mut databases: HashMap<String, Database> = HashMap::new();

pub struct Database {
    name: String,
    id: u16,
    collections: HashMap<String, Collection>,
}

pub struct Collection {
    name: String,
    id: u16,
    documents: HashMap<String, Document>,
    indexes: HashMap<String, Index>,
}

pub struct Document {
    _id: String,
    file_path: String,
}

pub struct Index {
    document_id: String,
    name: String,
    value: String,
}

pub unsafe fn create_database(database_name: String) {
    let database = Database {
        name: database_name,
        id: 0,
        collections: HashMap::new(),
    };
    let database_path = Path::new(&format!("/var/lib/uwu-database/data/databases/{}", database_name)).mkdirs();
    if !database_path.exists() {
        println!("Failed to create database: {}", database_name);
        return;
    }
    databases.insert(database.name.clone(), database);
}

pub unsafe fn create_collection(database_name: String, collection_name: String) -> Result<Collection, Error> {
    let database = databases.get_mut(&database_name).unwrap();
    let collection = Collection {
        name: collection_name,
        id: 0,
        documents: HashMap::new(),
        indexes: HashMap::new(),
    };
    let collection_path = Path::new(&format!("/var/lib/uwu-database/data/databases/{}/{}", database_name, collection_name)).mkdir();

    if !collection_path.exists() {
        Err(Error::custom(format!("Failed to create collection: {}", collection_name)))
    }
    let mut file = File::create(collection_path + "/indexes").unwrap();
    /*
    database id (1 byte)
    0x00
    collection id (1 byte)
    0x00
    index options (1 byte)
       0 id_field/custom field
       0 keep_duplicates
       0 unique
       0 in_memory
       0 nullable
       0 case_sensitive
       0
       0
    field name (? bytes/ascii)
     */
    let database_id: &[u8] = &[(database.id & 0xff) as u8, (database.id >> 8) as u8];
    //Creating the byte buffer to store the index
    let mut buffer: &[u8] = &[];
    buffer.write(database_id).unwrap();
    buffer.write(&[0x00]).unwrap();
    buffer.write(&[(collection.id & 0xff) as u8, (collection.id >> 8) as u8]).unwrap();
    buffer.write(&[0x00]).unwrap();
    buffer.write(&[0b10000000]).unwrap();
    buffer.write(&[0x00]).unwrap();
    //Write the buffer to the file
    file.write(buffer).unwrap();
    //Insert the collection into the database
    database.collections.insert(collection.name.clone(), collection);
}