# rust-metagenome-pangenome
- A single standalone application using RUST.
- analyzes all the genome and metagenomes from the pangenome point of view.
- does the metagenome assembly from the long reads and aligns them against the proteome. 
  - assembles the metagenome
  - genome completeness
  - genome annotation
  - analyzes the annotation and generate results
  - generates a PostGresSQL and Actix web for the generated results. 

```
cargo build 
```
Gaurav Sablok
