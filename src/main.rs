use std::io;
use rand::Rng;
use rand::distribution::Alphanumeric;

struct Patient {
    nom: String,
    prenom: String,
    date_naissance: u32,
    numero_telephone: u64,
    adresse: i32,
    identifiant : String,
    consultations: Vec<Consultation>,
}

struct Consultation {
    date: String,
    details: String,
}
fn  creation_identifiant() -> String {
    let mut  pid = rand::thread_rng();
    let id: String  = rng.sample_iter(&Alphanumeric).take(5).collect();
}


impl Patient {

    fn creation_patient() -> Patient {
        println!("entrez votre  nom  : ");
        let mut nom = String::new();
        io::stdin().read_line(&mut nom).expect("erreur  lors   de la  recuperaton");
        nom = nom.trim().to_string();

        println!("entrez votre   prenom : ");
        let mut prenom = String::new();
        io::stdin().read_line(&mut prenom).expect("erreur  lors   de la  recuperaton");
        prenom = prenom.trim().to_string();

        println!("entrez votre   date   de  naissance ");
        let mut date_naissance = String::new();
        io::stdin().read_line(&mut date_naissance).expect("ereur   de  recuperaton");
        let date_naissance: u32 = date_naissance.trim().parse().expect("Invalid input");

        println!("entrez votre  numero  de  telephone : ");
        let mut numero = String::new();
        io::stdin().read_line(&mut numero).expect("erreur  lors   de la  recuperaton");
        let numero: u64 = numero.trim().parse().expect("Invalid input");

        println!("entrez votre   adresse : ");
        let mut adresse = String::new();
        io::stdin().read_line(&mut adresse).expect("erreur  lors   de la  recuperaton");
        let adresse: i32 = adresse.trim().parse().expect("Invalid input");

        let  identifiant: String  = creation_identifiant();

        println!("voila  votre  identifiant  unique  : ", identifiant);

        Patient {
            nom,
            prenom,
            date_naissance,
            numero_telephone: numero,
            adresse,
            identifiant,
            consultations,
        }
    }

   fn  recherche_dossier(&self) -> Patient  {

        println!("entrez le nom et le prenom du patient que vous souhaiter rechercher");

        let  mut nom =  String::new();
        io::stdin().read_line(&mut nom).expect(" erreur  de  recuperation");
        let mut  nom : String = nom.trim().parse().expect("erreur  de lecture ");


        let  mut prenom =  String::new();
        io::stdin().read_line(&mut prenom).expect(" erreur  de  recuperation");
        let mut  prenom : String = prenom.trim().parse().expect("erreur  de lecture ");

        while self.nom == nom || self.prenom == prenom {
            println!("voicie  les  information complete du patient : {:?}" , Patient);
        }

        self.clone()
   }

   fn consultation(&mut self) {

        println!("Enter consultation date:");
        let mut date = String::new();
        io::stdin().read_line(&mut date).expect("Error reading input");
        date = date.trim().to_string();

        println!("Enter consultation details:");
        let mut details = String::new();
        io::stdin().read_line(&mut details).expect("Error reading input");
        details = details.trim().to_string();

        let consultation = Consultation {
            date,
            details,
        };

        self.consultations.push(consultation);
    }

    fn  suprimer_patient(&self) {

        println!("entrez  le nom  et le prenom  du patient  a suprimer ");

        let  mut prenom =  String::new();
        io::stdin().read_line(&mut prenom).expect(" erreur  de  recuperation");
        let mut  prenom : String = prenom.trim().parse().expect("erreur  de lecture ");

        let  mut nom =  String::new();
        io::stdin().read_line(&mut nom).expect(" erreur  de  recuperation");
        let mut  nom : String = nom.trim().parse().expect("erreur  de lecture ");

        if   self.nom == nom || self.prenom == prenom {
            drop(Patient)
        }else{
            println!("vous   avez mal ecrit les  identifiant ");
        }

        fn statistique_final(&mut self) -> Patient {

            println!("voila  les  donner  final du patient ");

            println!("entrez le nouveau nom : ");
            let mut nouveau_nom = String::new();
            io::stdin().read_line(&mut nouveau_nom).expect("erreur lors de la récupération");
            nouveau_nom = nouveau_nom.trim().to_string();
    
            println!("entrez le nouveau prénom : ");
            let mut nouveau_prenom = String::new();
            io::stdin().read_line(&mut nouveau_prenom).expect("erreur lors de la récupération");
            nouveau_prenom = nouveau_prenom.trim().to_string();
    
            println!("entrez la nouvelle date de naissance : ");
            let mut nouvelle_date_naissance = String::new();
            io::stdin().read_line(&mut nouvelle_date_naissance).expect("erreur lors de la récupération");
            let nouvelle_date_naissance: u32 = nouvelle_date_naissance.trim().parse().expect("Invalid input");
    
            println!("entrez le nouveau numéro de téléphone : ");
            let mut nouveau_numero = String::new();
            io::stdin().read_line(&mut nouveau_numero).expect("erreur lors de la récupération");
            let nouveau_numero: u64 = nouveau_numero.trim().parse().expect("Invalid input");
    
            println!("entrez la nouvelle adresse : ");
            let mut nouvelle_adresse = String::new();
            io::stdin().read_line(&mut nouvelle_adresse).expect("erreur lors de la récupération");
            let nouvelle_adresse: i32 = nouvelle_adresse.trim().parse().expect("Invalid input");
    
            self.nom = nouveau_nom;
            self.prenom = nouveau_prenom;
            self.date_naissance = nouvelle_date_naissance;
            self.numero_telephone = nouveau_numero;
            self.adresse = nouvelle_adresse;
        

            Patient {
                nom,
                prenom,
                date_naissance,
                numero_telephone: numero,
                adresse,
                identifiant,
                consultations,
            }
        }

    }

   
}
fn main() {
    println!("-------  Welcome to your consultation  --------------");

    let mut  patient =  Patient::new();

    println!("1 - cree un dossier  patient!" );
    println!("2 - rechercher un  dossier! ");
    println!("3 - consulter un  dossier!");
    println!("4 - suprimer le dossier!");
    println!("5 - mettre  le dossier a jour!");

    
    loop {
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading input");
        let choice: usize = choice.trim().parse().expect("Invalid input");

            match choice {
                1 => {
                    patient = Patient::creation_patient();
                }
                2 => {
                    patient = patient.recherche_dossier();
                }
                3 => {
                    patient.consultation();
                }
                4 => {
                    patient.supprimer_patient();
                }
                5 => {
                    patient = patient.statistique_final();
                }
                _ => {
                    println!("Invalid choice. Please try again.");
                }      
           }

        }

}