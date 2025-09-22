-- Table étudiant
CREATE TABLE etudiant (
    id SERIAL PRIMARY KEY,
    nom VARCHAR(100) NOT NULL,
    prenom VARCHAR(100) NOT NULL,
    login VARCHAR(100) UNIQUE NOT NULL,
    mot_de_passe VARCHAR(255) NOT NULL,
    adresse_mail VARCHAR(255) UNIQUE NOT NULL,
    type INT REFERENCES type_users(id) ON DELETE RESTRICT,
    deuxfa_secret VARCHAR(255) NULL
);

-- Table type de formation
CREATE TABLE type_formation (
    id SERIAL PRIMARY KEY,
    nom VARCHAR(100) NOT NULL
);

-- Table promo
CREATE TABLE promo (
    id SERIAL PRIMARY KEY,
    nom VARCHAR(100) NOT NULL,
    type_formation INT REFERENCES type_formation(id) ON DELETE RESTRICT,
    date_debut DATE,
    date_fin DATE,
    taille_min INT,
    taille_max INT
);

-- Relation n-n promo <-> étudiant
CREATE TABLE promo_etudiant (
    promo_id INT REFERENCES promo(id) ON DELETE RESTRICT,
    etudiant_id INT REFERENCES etudiant(id) ON DELETE RESTRICT,
    PRIMARY KEY (promo_id, etudiant_id)
);

-- Table université
CREATE TABLE universite (
    id SERIAL PRIMARY KEY,
    nom VARCHAR(255) NOT NULL
    login VARCHAR(100) UNIQUE NOT NULL,
    mot_de_passe VARCHAR(255) NOT NULL,
    adresse_mail VARCHAR(255) UNIQUE NOT NULL,
    type INT REFERENCES type_users(id) ON DELETE RESTRICT,
    deuxfa_secret VARCHAR(255) NULL
);

-- Relation université <-> promo
CREATE TABLE universite_promo (
    universite_id INT REFERENCES universite(id) ON DELETE RESTRICT,
    promo_id INT REFERENCES promo(id) ON DELETE RESTRICT,
    PRIMARY KEY (universite_id, promo_id)
);

-- Table entreprise
CREATE TABLE entreprise (
    id SERIAL PRIMARY KEY,
    nom VARCHAR(255) NOT NULL
    login VARCHAR(100) UNIQUE NOT NULL,
    mot_de_passe VARCHAR(255) NOT NULL,
    adresse_mail VARCHAR(255) UNIQUE NOT NULL,
    type INT REFERENCES type_users(id) ON DELETE RESTRICT,
    deuxfa_secret VARCHAR(255) NULL
);

-- Table stage
CREATE TABLE stage (
    id SERIAL PRIMARY KEY,
    type_formation INT REFERENCES type_formation(id) ON DELETE RESTRICT,
    entreprise_id INT REFERENCES entreprise(id) ON DELETE SET NULL,
    date_debut DATE,
    date_fin DATE,
    duree_min_stage INT,
    duree_max_stage INT,
    intitule VARCHAR(255),
    description TEXT,
    endroit VARCHAR(255)
);

-- Relation université <-> stage
CREATE TABLE universite_stage (
    universite_id INT REFERENCES universite(id) ON DELETE RESTRICT,
    stage_id INT REFERENCES stage(id) ON DELETE RESTRICT,
    PRIMARY KEY (universite_id, stage_id)
);
