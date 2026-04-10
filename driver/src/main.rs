use language_brainfuck as bf;
use language_better_brainfuck as bbf;
use language_simp as simp;
use transform_simp_brainfuck as simp_bf;
use transform_bf_bbf as bf_bbf;
use transform_bbf_bf as bbf_bf;
use transform_better_brainfuck_opt as bbf_opt;
use std::{env, fs};
use ir_core::{Compiler, Pipeline};



fn main() {
    // get program arguments
    let input_path = env::args().nth(1).expect("error: missing input file path");
    let output_path = env::args().nth(2).expect("error: missing output file path");

    let compiler = Compiler::new(
        simp::parser::SimpParser,

        Pipeline::from_transformations(vec![
            Box::new(simp_bf::SimpToBrainfuck::new()),
            Box::new(bf_bbf::BFToBBF::new()),
            Box::new(bbf_opt::BBFOptMerge::new(bbf::op::MOVE)),
            Box::new(bbf_opt::BBFOptMerge::new(bbf::op::ADD)),
            Box::new(bbf_bf::BBFToBF::new()),
        ]),

        bf::emitter::BrainfuckEmitter,
    );

    compile_file_to_file(&input_path, &output_path, compiler).expect("error: compilation failed");
}


fn compile_file_to_file(input_file: &str, output_file: &str, mut compiler: Compiler) -> Result<(), Box<dyn std::error::Error>> {

    let input = fs::read_to_string(input_file).expect("error: unable to read input file");

    let output = compiler.compile(&input)?;

    fs::write(output_file, output).expect("error: unable to write output file");
    Ok(())
}