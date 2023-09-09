use std::collections::HashMap;
use std::fmt::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

struct DatabaseList {
    data: Arc<Mutex<HashMap<String, Database>>>,
}

impl DatabaseList {
    fn new() -> Self {
        DatabaseList {
            data: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    fn insert(&self, key: String, value: Database) -> Option<Database> {
        let mut data = self.data.lock().unwrap();
        return data.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<Database> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }

    fn remove(&mut self, key: &String) -> Option<Database> {
        let mut data = self.data.lock().unwrap();
        return data.remove(key);
    }

    fn contains_key(&self, key: &String) -> bool {
        let data = self.data.lock().unwrap();
        return data.contains_key(key);
    }

    fn is_empty(&self) -> bool {
        let data = self.data.lock().unwrap();
        return data.is_empty();
    }

    fn len(&self) -> usize {
        let data = self.data.lock().unwrap();
        return data.len();
    }

    fn clear(&mut self) {
        let mut data = self.data.lock().unwrap();
        data.clear();
    }

}

lazy_static! {
    static ref DATABASES: DatabaseList = DatabaseList::new();
}

pub struct Database {
    pub(crate) name: String,
    pub(crate) id: u16,
    pub(crate) collections: HashMap<String, Collection>,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            name: self.name.clone(),
            id: self.id.clone(),
            collections: self.collections.clone(),
        }
    }
}

pub struct Collection {
    pub(crate) name: String,
    pub(crate) id: u16,
    pub(crate) documents: HashMap<String, Document>,
    pub(crate) indexes: HashMap<String, Index>,
}

impl Clone for Collection {
    fn clone(&self) -> Self {
        Collection {
            name: self.name.clone(),
            id: self.id.clone(),
            documents: self.documents.clone(),
            indexes: self.indexes.clone(),
        }
    }
}

pub struct Document {
    pub(crate) _id: String,
    pub(crate) file_path: String,
}

impl Clone for Document {
    fn clone(&self) -> Self {
        Document {
            _id: self._id.clone(),
            file_path: self.file_path.clone(),
        }
    }
}

pub struct Index {
    pub(crate) document_id: String,
    pub(crate) name: String,
    pub(crate) value: String,
}

impl Clone for Index {
    fn clone(&self) -> Self {
        Index {
            document_id: self.document_id.clone(),
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }
}

pub unsafe fn create_database(database_name: String) {
    let database = Database {
        name: database_name.clone(),
        id: 0,
        collections: HashMap::new(),
    };
    let mut path = &format!("/var/lib/uwu-database/data/databases/{}", database_name);
    let database_path = Path::new(path);

    if !database_path.exists() {
        println!("Failed to create database: {}", database_name);
        return;
    }
    DATABASES.insert(database.name.clone(), database).unwrap();
}

pub unsafe fn create_collection(database_name: String, collection_name: String) -> Result<u16, Error> {
    let mut database = DATABASES.get(&database_name).unwrap();
    let collection = Collection {
        name: collection_name.clone(),
        id: 0,
        documents: HashMap::new(),
        indexes: HashMap::new(),
    };
    let path = &format!("/var/lib/uwu-database/data/databases/{}/{}", database_name, collection_name);
    let collection_path = Path::new(path);

    if !collection_path.exists() {
        eprintln!("Failed to create collection: {}", collection_name);
        return Err(Error);
    }
    let mut file = File::create(collection_path.with_file_name("indexes")).unwrap();
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
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut current_index: usize = 0;
    //Write the database id to the buffer
    database_id.iter().for_each(|byte| {
        buffer[current_index] = *byte;
        current_index += 1;
    });
    buffer[current_index] = 0x00;
    current_index += 1;
    //Write the collection id to the buffer
    let collection_id: &[u8] = &[(collection.id & 0xff) as u8, (collection.id >> 8) as u8];
    collection_id.iter().for_each(|byte| {
        buffer[current_index] = *byte;
        current_index += 1;
    });
    buffer[current_index] = 0x00;
    current_index += 1;
    //Write the index options to the buffer
    buffer[current_index] = 0b00000000;
    current_index += 1;
    //Write the field name to the buffer
    let field_name: &[u8] = &collection.name.as_bytes();
    field_name.iter().for_each(|byte| {
        buffer[current_index] = *byte;
        current_index += 1;
    });
    //Write the buffer to the file
    file.write(&buffer).unwrap();
    //Insert the collection into the database
    let collection_id = collection.id;
    database.collections.insert(collection.name.clone(), collection);
    Ok(collection_id)
}

pub fn get_database(database_name: String) -> Option<Database> {
    DATABASES.get(&database_name)
}

pub fn get_collection(database_name: &String, collection_name: String) -> Option<Collection> {
    let database = DATABASES.get(&database_name).unwrap();
    database.collections.get(&collection_name).cloned()
}

