use halo2_proofs::halo2curves::{bn256::Fr, pasta::Fp};
use zkml::{
  model::ModelCircuit,
  utils::{proving_ipa::time_circuit_ipa, proving_kzg::time_circuit_kzg},
};

fn main() {

  let model = String::from("custom");
  let mut config_fname= "";
  let mut inp_fname ="";
  let mut kzg_or_ipa = "";

  if model == "mnist"
  {
    //config_fname = "pw_examples/mnist/model.msgpack";
    //inp_fname = "pw_examples/mnist/example_inp.msgpack"
    config_fname= "examples/mnist/model.msgpack";
    inp_fname ="examples/mnist/inp.msgpack";
    kzg_or_ipa = "kzg";
  } 
  else if model == "clip" {
    config_fname= "examples/nlp/clip/model.msgpack";
    inp_fname ="examples/nlp/clip/inp.msgpack";
    kzg_or_ipa = "kzg"; 
  }//
  else if model == "resnet" {
    config_fname= "pw_examples/resnet/converted_model.msgpack";
    inp_fname ="pw_examples/resnet/example_inp.msgpack";
    kzg_or_ipa = "kzg"; 
  }
  else if model == "squeezenet" {
    config_fname= "pw_examples/squeezenet/converted_model.msgpack";
    inp_fname ="pw_examples/squeezenet/example_inp.msgpack";
    kzg_or_ipa = "kzg"; 
  }
  else if model == "custom" {
    config_fname= "pw_examples/custom2/model.msgpack";
    inp_fname ="pw_examples/custom2/inp.msgpack";
    kzg_or_ipa = "kzg"; 
  }
  /* 

  let config_fname = std::env::args().nth(1).expect("config file path");
  let inp_fname = std::env::args().nth(2).expect("input file path");
  let kzg_or_ipa = std::env::args().nth(3).expect("kzg or ipa");

*/

if kzg_or_ipa != "kzg" && kzg_or_ipa != "ipa" {
    panic!("Must specify kzg or ipa");
  }

  if kzg_or_ipa == "kzg" {
    let circuit = ModelCircuit::<Fr>::generate_from_file(&config_fname, &inp_fname);
    time_circuit_kzg(circuit,model);
  } else {
    let circuit = ModelCircuit::<Fp>::generate_from_file(&config_fname, &inp_fname);
    time_circuit_ipa(circuit);
  }
}
