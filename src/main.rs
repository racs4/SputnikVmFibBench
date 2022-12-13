use evm::backend::{MemoryAccount, MemoryBackend, MemoryVicinity};
use evm::executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata};
use evm::Config;
// use kindelia::constants::GENESIS_CODE;
// use kindelia::hvm::{parse_code, Runtime, StatementInfo};
use primitive_types::{H160, U256};
use std::fmt::Display;
use std::ops::{Add, AddAssign};
use std::time::Instant;
use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, Default)]
struct BenchResult {
    time: f64,
    gas: u64,
    gas_per_time: f64,
}

impl BenchResult {
    fn new(time: f64, gas: u64) -> Self {
        let gas_per_time = gas as f64 / time;

        BenchResult {
            time,
            gas,
            gas_per_time,
        }
    }
}

impl Display for BenchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "time: {}s\ngas: {}\ngas/time: {} gas/s",
            self.time, self.gas, self.gas_per_time
        )
    }
}

impl Add for BenchResult {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        BenchResult {
            gas: self.gas + rhs.gas,
            time: self.time + rhs.time,
            gas_per_time: self.gas_per_time + rhs.gas_per_time,
        }
    }
}

impl AddAssign for BenchResult {
    fn add_assign(&mut self, rhs: Self) {
        self.gas += rhs.gas;
        self.gas_per_time += rhs.gas_per_time;
        self.time += rhs.time;
    }
}

/// Run Sputnik's benchmark. The contract of the generated code
/// is present in `src/sol` folder.
///
/// This benchmark model was taken fron the original repo:
/// https://github.com/rust-blockchain/evm/blob/master/benches/loop.rs
fn evm() -> BenchResult {
    let config = Config::london();

    let vicinity = MemoryVicinity {
        gas_price: U256::zero(),
        origin: H160::default(),
        block_hashes: Vec::new(),
        block_number: Default::default(),
        block_coinbase: Default::default(),
        block_timestamp: Default::default(),
        block_difficulty: Default::default(),
        block_gas_limit: Default::default(),
        chain_id: U256::one(),
        block_base_fee_per_gas: U256::zero(),
    };

    let mut state = BTreeMap::new();
    state.insert(
		H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
		MemoryAccount {
			nonce: U256::one(),
			balance: U256::from(10000000),
			storage: BTreeMap::new(),
			code: hex::decode("608060405234801561001057600080fd5b506004361061002b5760003560e01c8063f1820bdc14610030575b600080fd5b61004361003e3660046100b9565b610055565b60405190815260200160405180910390f35b600061006082610066565b92915050565b60008160ff1660000361007b57506000919050565b8160ff1660010361008e57506001919050565b6100a161009c6002846100f9565b610066565b6100af61009c6001856100f9565b6100609190610112565b6000602082840312156100cb57600080fd5b813560ff811681146100dc57600080fd5b9392505050565b634e487b7160e01b600052601160045260246000fd5b60ff8281168282160390811115610060576100606100e3565b80820180821115610060576100606100e356fea2646970667358221220c578eff35e02a404952013ffd2502337ec8e109b44458c5a47f50700949967b564736f6c63430008110033").unwrap(),
		}
	);
    state.insert(
        H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
        MemoryAccount {
            nonce: U256::one(),
            balance: U256::from(10000000),
            storage: BTreeMap::new(),
            code: Vec::new(),
        },
    );

    let backend = MemoryBackend::new(&vicinity, state);
    let metadata = StackSubstateMetadata::new(u64::MAX, &config);
    let state = MemoryStackState::new(metadata, &backend);
    let precompiles = BTreeMap::new();
    let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

    let input =
        hex::decode("f1820bdc000000000000000000000000000000000000000000000000000000000000002a")
            .unwrap();
    let time = Instant::now();
    executor.transact_call(
        H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
        H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
        U256::zero(),
        input,
        u64::MAX,
        Vec::new(),
    );

    // println!("{:?}", reason);
    // println!("{:?}", String::from_utf8(a).unwrap());

    let time = time.elapsed().as_secs_f64();

    let gas = executor.used_gas();

    BenchResult::new(time, gas)
}

/// Creates a Kindelia Runtime with the genesis and
/// pre function code.
// fn pre_kdl() -> Runtime {
//     // rt generation
//     let genesis_statements = parse_code(GENESIS_CODE).unwrap();
//     let mut rt = kindelia::hvm::init_runtime("~/.kindelia".parse().unwrap(), &genesis_statements);

//     // function declaration
//     let pre_statements = parse_code(include_str!("kdl/pre_fib.kdl")).unwrap();
//     rt.run_statements(&pre_statements, true, true);
//     rt
// }

/// Runs the kdl benchmark. The code runned here is taken
/// from `src/kdl/fib.kdl`
// fn kdl(rt: &mut Runtime) -> BenchResult {
//     // running loop
//     let statements = parse_code(include_str!("kdl/fib.kdl")).unwrap();
//     let time = Instant::now();
//     let results = rt.run_statements(&statements, true, true);
//     let time = time.elapsed().as_secs_f64();

//     // get used gas
//     let result = results.first().unwrap(); // only one run is executed, so there is only one result
//     let result = result.as_ref().unwrap(); // it is expected
//     let gas = {
//         match result {
//             StatementInfo::Run { used_mana, .. } => used_mana,
//             _ => panic!("Not a run"),
//         }
//     };

//     //
//     BenchResult::new(time, *gas)
// }

fn main() {
    // env_logger::init();
    let n = 1;

    let mut evm_results = Vec::new();
    // let mut kdl_results = Vec::new();

    // let mut rt = pre_kdl();

    for i in 0..n {
        println!("Running {}", i);
        evm_results.push(evm());
        // kdl_results.push(kdl(&mut rt));
    }

    // let gas: u64 = evm_results.iter().map(|b_result| b_result.gas).sum();

    println!("{:?}", evm_results);
    // println!("{:?}", kdl_results);
}
