mod args;
use args::PangenomeArgs;
use clap::Parser;
use cmd_lib::run_fun;
use cmd_lib::run_cmd;
use crate::env::set_current_dir;
use std::fs;
use std::error::Error;
use std::env;
use std::process::Command;
use std::path::Path;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
#[allow(dead_code)]

/*
 *Author Gaurav Sablok
 *Universitat Potsdam
 *Date 2024-11-15

* a rustlang application to generate and scaffold the pangenome from the hifi reads either using
* the hifiasm and then annotate the pangenome using the given protein and then extract the aligned
* regions from the annotated pangenome.
*
*
* */

fn main() {

    let args = PangenomeArgs::parse();
    pangenome(&args.reads_arg, &args.genome_arg, &args.thread_arg, &args.protein_arg);
    make_fasta();
    genome_complete();
    genome_annotation(&args.protein_arg);
}

// assembling the genome for the pangenome - lifetime borrows
fn pangenome<'a>(path: &'a str, genome: &'a str, thread: &'a i32, proteinfasta: &'a str) {
    fs::create_dir("pangenome_assemble");
    let assemblerpath = Path::new("./pangenome_assemble");
    set_current_dir(&assemblerpath);
    run_cmd!("git clone https://github.com/chhylp123/hifiasm");
    run_cmd!("cd hifiasm && make");
    let reads = &path.to_string();
    let genomeassembly = std::process::Command::new("./hifiasm")
        .arg("-o genome")
        .arg("-t &thread")
        .arg("-f0")
        .arg(&reads)
        .arg("2>")
        .arg("genome-assembly.log")
        .spawn()
        .expect("hifiasm failed to run the assembly");
}

// converting the graph to the fasta and a linearized fasta for genome annotation
fn make_fasta() {
   let genomeassembly = run_fun!(
       bash -c "mv genome.bp.p_ctg.gfa assembled-genome.gfa"
        | awk r#"awk '/^S/{print ">"$2;print $3}' assembled-genome.gfa > assembled-genome.fasta"#
        | awk r#"'/^>/ {printf("\n%s\n",$0);next; } { printf("%s",$0);} END {printf("\n");}'
                                             assembled-genome.fasta > final-genome-assembled.fasta"#
       );
   println!(r#"genome assembly linear fasta failed and the assembled genome
                                     in the graph format is present in the assembled-genome.gfa"#);
}

// genome completeness
fn genome_complete() {
    fs::create_dir("./genome_completeness");
    let path_genome = Path::new("./pangenome/genome_completeness");
    let final_assembly = String::from("./pangenome/genome_completeness/final-genome-assembled.fasta");
    set_current_dir(&path_genome);
    let compleasm = run_fun!(bash -c "wget https://github.com/huangnengCSU/compleasm/releases/download/v0.2.6/compleasm-0.2.6_x64-linux.tar.bz2"
        | bash -c "tar -jxvf compleasm-0.2.6_x64-linux.tar.bz2)"
     );
    set_current_dir("./pangenome/genome_completeness/compleasm");
    let compleasum_run = run_cmd!(
          bash -c "cp -r ../../final-genome-assembled.fasta ./"
    );
    let genome_assessment = std::process::Command::new("python3")
              .arg("compleasm.py")
              .arg("run")
              .arg("-t")
              .arg("10")
              .arg("-l")
              .arg("eukaryota")
              .arg("-a")
              .arg(&final_assembly)
              .arg("-o")
              .arg("final-genome-completness")
              .spawn()
              .ok()
              .expect("completeness failed");
}

fn genome_annotation(proteinfasta: &str) {
    fs::create_dir_all("./pangenome/genome_annotations");
    let annotation_path = Path::new("./pangenome/genome_annotations");
    set_current_dir(&annotation_path);
    run_func!(
        bash -c "git clone https://github.com/lh3/miniprot"
        | cd miniprot && make
        | mv miniprot ../
        | echo "miniprot has been installed"
    );
    let final_assembly = String::from("./pangenome/genome_completeness/final-genome-assembled.fasta");
    let protein_annotations = std::process::Command::new("miniprot")
           .arg("-t")
           .arg("10")
           .arg("--gff")
           .arg(&final_assembly)
           .arg(&proteinfasta)
           .arg(">")
           .arg("final-genome-annotations.gff")
           .spawn()
           .ok()
           .expect("genome annotations failed");
}

// started to code the actix web for the same. It will generate all API for the same.

fn analyze_alignments(alignment: &str) {
  let mut file = File::open(&).expect("file not found");
  let mut reader = BufReader::new(&file);


}





fn actix_generate() {


}
