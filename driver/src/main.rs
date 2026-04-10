use language_brainfuck as bf;
use language_better_brainfuck as bbf;
use language_simp as simp;
use transform_simp_brainfuck as simp_bf;
use transform_bf_bbf as bf_bbf;
use transform_bbf_bf as bbf_bf;
use transform_better_brainfuck_opt as bbf_opt;
use std::{env, fs};
use ir_core::{Pipeline};



fn main() {
    // get program arguments
    let input_path = env::args().nth(1).expect("error: missing input file path");
    let output_path = env::args().nth(2).expect("error: missing output file path");

    compile_simp_to_bf(&input_path, &output_path);
}


fn compile_simp_to_bf(input_file: &str, output_file: &str) {
    
    let mut pipeline = Pipeline::new();
    pipeline.add_transformation(simp_bf::SimpToBrainfuck::new());
    pipeline.add_transformation(bf_bbf::BFToBBF::new());
    pipeline.add_transformation(bbf_opt::BBFOptMerge::new(bbf::op::MOVE));
    pipeline.add_transformation(bbf_opt::BBFOptMerge::new(bbf::op::ADD));
    pipeline.add_transformation(bbf_bf::BBFToBF::new());

    let input = fs::read_to_string(input_file).expect("error: unable to read input file");
    let input = simp::parser::parse(&input).expect("error: unable to parse");
    let result = pipeline.run(input);
    let output = bf::emitter::emit(&result.instructions).expect("error: unable to emit output");

    fs::write(output_file, output).expect("error: unable to write output file");
}