# rust-metagenome-pangenome
- a rust metagenome-pangenome analyzer 
- analyzes all the metagenomes from the pangenome point.
- does the meategnome assembly from the long reads and aligns them against the proteome.
- This follows the following steps: 
  - assembles the metagenome
  - genome completeness
  - genome annotation
  - analyzes the annotation and generate results
  - generates a PostGresSQL and Actix web for the generated results. 
- A single standalone application using RUST. 

Gaurav Sablok
