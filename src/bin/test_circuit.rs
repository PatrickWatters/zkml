use halo2_proofs::{dev::MockProver, halo2curves::bn256::Fr};
use zkml::{
  model::ModelCircuit,
  utils::{
    helpers::get_public_values,
    loader::{load_model_msgpack, ModelMsgpack},
  },
};

fn main() {

  //let config_fname = "pw_examples/mnist/model.msgpack";
  //let inp_fname = "pw_examples/mnist/example_inp.msgpack";
  
  //let config_fname= "examples/nlp/clip/model.msgpack";
  //let inp_fname = "examples/nlp/clip/inp.msgpack";
  
  //let config_fname = "pw_examples/resnet/converted_model.msgpack";
  //let inp_fname = "pw_examples/resnet/example_inp.msgpack";

  let config_fname = "pw_examples/custom/model.msgpack";
  let inp_fname = "pw_examples/custom/inp.msgpack";

  let config: ModelMsgpack = load_model_msgpack(&config_fname, &inp_fname);

  let circuit = ModelCircuit::<Fr>::generate_from_file(&config_fname, &inp_fname);

  let _prover = MockProver::run(config.k.try_into().unwrap(), &circuit, vec![vec![]]).unwrap();
  let public_vals = get_public_values();

  let prover = MockProver::run(config.k.try_into().unwrap(), &circuit, vec![public_vals]).unwrap();
  assert_eq!(prover.verify(), Ok(()));
}
