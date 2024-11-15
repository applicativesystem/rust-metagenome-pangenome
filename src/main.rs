mod args;
use std::io;
use args::PangenomeArgs;
use clap::Parser;
use cmd_lib::run_fun;
use cmd_lib::run_cmd;
use crate::env::set_current_dir;
use std::fs;
use shellfn::shell;
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
* */

fn main() {

    let args = PangenomeArgs::parse();
    pangenome(&args.reads_arg, &args.genome_arg, &args.thread_arg, &args.protein_arg);
}

fn pangenome(path: &str, genome: &str, thread: &i32, proteinfasta: &str) -> Result<(),std::io::Error> {
    fs::create_dir("pangenome_assemble");
    let assemblerpath = Path::new("./pangenome_assemble");
    set_current_dir(&assemblerpath);
    run_cmd!("git clone https://github.com/chhylp123/hifiasm");
    run_cmd!("cd hifiasm && make");
    let reads = &path.to_string();
    let genomeassembly = Command::new("./hifiasm")
        .arg("-o genome")
        .arg("-t &thread")
        .arg("-f0")
        .arg(&reads)
        .arg("2>")
        .arg("genome-assembly.log")
        .spawn()
        .output()
        .expect("hifiasm failed to run the assembly")
}

fn make_fasta() {
   let genomeassembly = run_fun!(
       bash -c "mv genome.bp.p_ctg.gfa assembled-genome.gfa"
        | awk r#"awk '/^S/{print ">"$2;print $3}' assembled-genome.gfa > assembled-genome.fasta"#
        | awk r#"'/^>/ {printf("\n%s\n",$0);next; } { printf("%s",$0);} END {printf("\n");}' assembled-genome.fasta > final-genome-assembled.fasta"#
       );
info!("genome assembly linear fasta failed and the assembled genome in the graph format is present in the assembled-genome.gfa");
}
