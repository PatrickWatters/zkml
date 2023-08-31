use std::{
  fs::File,
  io::{BufReader, Write},
  path::Path,
  time::Instant,
  error::Error,
};



use halo2_proofs::{
  dev::MockProver,
  halo2curves::bn256::{Bn256, Fr, G1Affine},
  plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, VerifyingKey},
  poly::{
    commitment::Params,
    kzg::{
      commitment::{KZGCommitmentScheme, ParamsKZG},
      multiopen::{ProverSHPLONK, VerifierSHPLONK},
      strategy::SingleStrategy,
    },
  },
  transcript::{
    Blake2bRead, Blake2bWrite, Challenge255, TranscriptReadBuffer, TranscriptWriterBuffer,
  },
  SerdeFormat,
};

use crate::{model::ModelCircuit, utils::helpers::get_public_values};

pub fn get_kzg_params(params_dir: &str, degree: u32) -> ParamsKZG<Bn256> {
  let rng = rand::thread_rng();
  let path = format!("{}/{}.params", params_dir, degree);
  let params_path = Path::new(&path);
  if File::open(&params_path).is_err() {
    let params = ParamsKZG::<Bn256>::setup(degree, rng);
    let mut buf = Vec::new();

    params.write(&mut buf).expect("Failed to write params");
    let mut file = File::create(&params_path).expect("Failed to create params file");
    file
      .write_all(&buf[..])
      .expect("Failed to write params to file");
  }

  let mut params_fs = File::open(&params_path).expect("couldn't load params");
  let params = ParamsKZG::<Bn256>::read(&mut params_fs).expect("Failed to read params");
  params
}

pub fn serialize(data: &Vec<u8>, path: &str) -> u64 {
  let mut file = File::create(path).unwrap();
  file.write_all(data).unwrap();
  file.metadata().unwrap().len()
}

pub fn verify_kzg(
  params: &ParamsKZG<Bn256>,
  vk: &VerifyingKey<G1Affine>,
  strategy: SingleStrategy<Bn256>,
  public_vals: &Vec<Fr>,
  mut transcript: Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
) {
  assert!(
    verify_proof::<
      KZGCommitmentScheme<Bn256>,
      VerifierSHPLONK<'_, Bn256>,
      Challenge255<G1Affine>,
      Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
      halo2_proofs::poly::kzg::strategy::SingleStrategy<'_, Bn256>,
    >(&params, &vk, strategy, &[&[&public_vals]], &mut transcript)
    .is_ok(),
    "proof did not verify"
  );
}



struct LoggingInfo {
  model_name: String,
  num_constraints: String,
  params_construction: String,
  generating_vkey: String,
  vkey_size: String,
  generating_pkey: String,
  pkey_size: String,
  filling_circuit: String,
  proving_time: String,
  proof_size: String,
  verifying_time: String,
}

pub fn time_circuit_kzg(circuit: ModelCircuit<Fr>, model:String) {

  let mut stat_collector = LoggingInfo{
    model_name:String::from(""),
    num_constraints:String::from(""),
    params_construction:String::from(""),
    generating_vkey:String::from(""), 
    vkey_size:String::from(""), 
    generating_pkey:String::from(""),
    pkey_size:String::from(""), 
    filling_circuit:String::from(""), 
    proving_time:String::from(""), 
    proof_size:String::from(""), 
    verifying_time:String::from(""), 
};
  stat_collector.model_name = model;


  stat_collector.num_constraints = format!("{}",1 << circuit.k as u64); 

  let rng = rand::thread_rng();
  let start = Instant::now();

  let degree = circuit.k as u32;
  let params = get_kzg_params("./params_kzg", degree);

  let circuit_duration = start.elapsed();
  stat_collector.params_construction = format!("{}",circuit_duration.as_millis());

  println!(
    "Time elapsed in params construction: {:?}",
    circuit_duration
  );

  let vk_circuit = circuit.clone();
  let vk = keygen_vk(&params, &vk_circuit).unwrap();
  drop(vk_circuit);
  let vk_duration = start.elapsed();
  stat_collector.generating_vkey = format!("{}",vk_duration.as_millis());

  println!(
    "Time elapsed in generating vkey: {:?}",
    vk_duration - circuit_duration
  );

  let vkey_size = serialize(&vk.to_bytes(SerdeFormat::RawBytes), "vkey");
  stat_collector.vkey_size = format!("{}",vkey_size);

  println!("vkey size: {} bytes", vkey_size);

  let pk_circuit = circuit.clone();
  let pk = keygen_pk(&params, vk, &pk_circuit).unwrap();
  let pk_duration = start.elapsed();
  stat_collector.generating_pkey = format!("{}",pk_duration.as_millis());

  println!(
    "Time elapsed in generating pkey: {:?}",
    pk_duration - vk_duration
  );
  drop(pk_circuit);

  let pkey_size = serialize(&pk.to_bytes(SerdeFormat::RawBytes), "pkey");
  println!("pkey size: {} bytes", pkey_size);
  stat_collector.pkey_size = format!("{}",vkey_size);

  let fill_duration = start.elapsed();

  let proof_circuit = circuit.clone();
  let _prover = MockProver::run(degree, &proof_circuit, vec![vec![]]).unwrap();
  let public_vals = get_public_values();

  let filling: std::time::Duration = fill_duration - pk_duration;
  stat_collector.filling_circuit = format!("{}",filling.as_millis());

  println!(
    "Time elapsed in filling circuit: {:?}",
    filling
  );

  // Convert public vals to serializable format
  let public_vals_u8: Vec<u8> = public_vals
    .iter()
    .map(|v: &Fr| v.to_bytes().to_vec())
    .flatten()
    .collect();
  let public_vals_u8_size = serialize(&public_vals_u8, "public_vals");
  println!("Public vals size: {} bytes", public_vals_u8_size);

  let mut transcript = Blake2bWrite::<_, G1Affine, Challenge255<_>>::init(vec![]);
  create_proof::<
    KZGCommitmentScheme<Bn256>,
    ProverSHPLONK<'_, Bn256>,
    Challenge255<G1Affine>,
    _,
    Blake2bWrite<Vec<u8>, G1Affine, Challenge255<G1Affine>>,
    ModelCircuit<Fr>,
  >(
    &params,
    &pk,
    &[proof_circuit],
    &[&[&public_vals]],
    rng,
    &mut transcript,
  )
  .unwrap();
  let proof = transcript.finalize();
  let proof_duration = start.elapsed();

  let prooftime: std::time::Duration = proof_duration - fill_duration;
  stat_collector.proving_time = format!("{}",prooftime.as_millis());
  println!("Proving time: {:?}", prooftime);

  let proof_size = serialize(&proof, "proof");
  let proof = std::fs::read("proof").unwrap();

  stat_collector.proof_size = format!("{}",proof_size);
  println!("Proof size: {} bytes", proof_size);

  let strategy = SingleStrategy::new(&params);
  let transcript_read = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);

  println!("public vals: {:?}", public_vals);
  verify_kzg(
    &params,
    &pk.get_vk(),
    strategy,
    &public_vals,
    transcript_read,
  );
  let verify_duration = start.elapsed();

  let veriftime: std::time::Duration = verify_duration - proof_duration;
  stat_collector.verifying_time = format!("{}",veriftime.as_millis());

  println!("Verifying time: {:?}", veriftime);

  let _ = log_stats(stat_collector);
}

// Standalone verification
pub fn verify_circuit_kzg(
  circuit: ModelCircuit<Fr>,
  vkey_fname: &str,
  proof_fname: &str,
  public_vals_fname: &str,
) {
  let degree = circuit.k as u32;
  let params = get_kzg_params("./params_kzg", degree);
  println!("Loaded the parameters");

  let vk = VerifyingKey::read::<BufReader<File>, ModelCircuit<Fr>>(
    &mut BufReader::new(File::open(vkey_fname).unwrap()),
    SerdeFormat::RawBytes,
    (),
  )
  .unwrap();
  println!("Loaded vkey");

  let proof = std::fs::read(proof_fname).unwrap();

  let public_vals_u8 = std::fs::read(&public_vals_fname).unwrap();
  let public_vals: Vec<Fr> = public_vals_u8
    .chunks(32)
    .map(|chunk| Fr::from_bytes(chunk.try_into().expect("conversion failed")).unwrap())
    .collect();

  let strategy = SingleStrategy::new(&params);
  let transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);

  let start = Instant::now();
  let verify_start = start.elapsed();
  verify_kzg(&params, &vk, strategy, &public_vals, transcript);
  let verify_duration = start.elapsed();
  println!("Verifying time: {:?}", verify_duration - verify_start);
  println!("Proof verified!")
}

fn log_stats(stat_collector:LoggingInfo)-> Result<(), Box<dyn Error>>
{ 
    let filename = "/home/project2reu/patrick/gpuhalo2/halo2/stats/zkml_stats.csv";
    let already_exists= Path::new(filename).exists();

    let file = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .append(true)
    .open(filename)
    .unwrap();

    let mut wtr = csv::Writer::from_writer(file);
    
    if already_exists == false
    {
        wtr.write_record(&["model","num_constraints","params_construction(ms)", "gen_vkey(ms)", "vkey_size(bytes)",
        "gen_pkey(ms)", "pkey_size(bytes)", "filling_circuit(ms)", "proving_time(ms)", "proof_size(bytes)","verif_time(ms)"])?;    
    }

    wtr.write_record(&[stat_collector.model_name, stat_collector.num_constraints, stat_collector.params_construction, stat_collector.generating_vkey,
       stat_collector.vkey_size,stat_collector.generating_pkey,stat_collector.pkey_size,stat_collector.filling_circuit,
       stat_collector.proving_time,stat_collector.proof_size,stat_collector.verifying_time])?;
    wtr.flush()?;
    Ok(())    
}
