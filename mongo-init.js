// Sélection de la base
db = db.getSiblingDB("file_metadata");

// Création collection
db.createCollection("files");

// Index uniques
db.files.createIndex({ "checksum": 1 }, { unique: true });
db.files.createIndex({ "path": 1 }, { unique: true });

// Index de recherche
db.files.createIndex({ "owner": 1 });
db.files.createIndex({ "createdAt": -1 });
db.files.createIndex({ "objectName": 1 });
db.files.createIndex({ "bucket": 1 });

// Document exemple (optionnel)
db.files.insertOne({
  _id: "00000000-0000-0000-0000-000000000001",
  objectName: "example.pdf",
  bucket: "documents",
  volume: "files",
  size: 123456,
  mimeType: "application/pdf",
  checksum: "sha256:example",
  owner: "system",
  createdAt: new Date(),
  path: "files/documents/example.pdf"
});
