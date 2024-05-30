use miden_verifier::{verify, ProgramInfo, Kernel};
use std::env;
use std::path::PathBuf;
use std::path::Path;
// use miden::verify;
use crate::helpers::ProgramHash;
use crate::helpers::InputFile;
use crate::helpers::OutputFile;
use crate::helpers::ProofFile;
mod helpers;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <program_hash> <proof_path> <stack_inputs> <stack_outputs>", args[0]);
        std::process::exit(1);
    }

    // read program hash from input
    let program_hash = ProgramHash::read(&args[0]).unwrap();


    // load input data from file
    let mut input_path = PathBuf::new();

    input_path.push(args[2].clone());

    let proof_path = Path::new(&args[1]);

    let input_data = InputFile::read(&Some(input_path), proof_path).unwrap();
    
    // fetch the stack inputs from the arguments
    let stack_inputs = input_data.parse_stack_inputs().unwrap();
    
    // load outputs data from file

    let mut output_path = PathBuf::new();

    output_path.push(args[3].clone());

    let outputs_data = OutputFile::read(&Some(output_path), proof_path).unwrap();
    
    let stack_outputs = outputs_data.stack_outputs().unwrap();

    // load proof from file
    let proof = ProofFile::read(&Some(proof_path.to_path_buf()), proof_path).unwrap();

    // This is copied from core midenvm verifier.
    // TODO accept kernel as CLI argument -- this is not done in core midenVM
    let kernel = Kernel::default();
    let program_info = ProgramInfo::new(program_hash, kernel);
    
    // verify proof
    let result = verify(program_info, stack_inputs, stack_outputs, proof)
        .map_err(|err| format!("Program failed verification! - {}", err));

    // TODO: what to put here?
    // let output: HyleOutput<()> = result ????

}

