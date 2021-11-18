use zk_evm::testing::*;
use super::*;

pub const ENTRY_POINT_PAGE: u32 = 4;
pub const INITIAL_TIMESTAMP: u32 = 8;
pub const INITIAL_MEMORY_COUNTER: u32 = 8;
pub const DEFAULT_CALLER: &'static str = "3000";
pub const DEFAULT_CALLEE: &'static str = "5000";

pub fn address_from_str_radix(str: &str, radix: u32) -> Address {
    use num_traits::Num;
    let value = num_bigint::BigUint::from_str_radix(str, radix).unwrap();
    let be_bytes = value.to_bytes_be();
    if be_bytes.len() > 20 {
        panic!("Address is too long");
    }

    let mut new = Address::default();
    new.as_bytes_mut()[(20 - be_bytes.len())..].copy_from_slice(&be_bytes);

    new
}

pub fn create_default_block_properties() -> BlockProperties {
    BlockProperties {
        block_number: 1u64,
        block_timestamp: 123456789u64,
        coinbase: Address::zero(),
        pubdata_byte_price_in_ergs: 1,
        storage_cold_access_price_in_ergs: 100,
        storage_warm_access_refund_in_ergs: 90,
        storage_write_price_in_ergs: 1000,
        zkporter_is_available: true,
    }
}

pub fn create_vm_with_default_settings<
    'a, const B: bool
>(
    tools: &'a mut BasicTestingTools<B>, 
    block_properties: &'a BlockProperties
) -> VmState<'a, InMemoryStorage, SimpleMemory, InMemoryEventSink, DefaultPrecompilesProcessor<B>, SimpleDecommitter<B>, DummyTracer> {

    let mut vm = VmState::empty_state(
        &mut tools.storage, 
        &mut tools.memory, 
        &mut tools.event_sink, 
        &mut tools.precompiles_processor, 
        &mut tools.decommittment_processor, 
        &mut tools.witness_tracer, 
        block_properties
    );

    let bootloader_context = CallStackEntry {
        contract_address: address_from_str_radix(DEFAULT_CALLEE, 10),
        msg_sender: address_from_str_radix(DEFAULT_CALLER, 10),
        base_memory_page: MemoryPage(5),
        code_page: MemoryPage(ENTRY_POINT_PAGE),
        calldata_page: MemoryPage(3),
        calldata_offset: MemoryOffset(0u16),
        calldata_len: MemoryOffset(0u16),
        returndata_page: MemoryPage(0),
        returndata_offset: MemoryOffset(0),
        returndata_len: MemoryOffset(0),
        sp: MemoryIndex(0u16),
        pc: MemoryIndex(0u16),
        exception_handler_location: MemoryIndex(0u16),
        ergs_remaining: u32::MAX,
        shard_id: 0u8,
        is_static: false,
        is_local_frame: false,
    };

    // we consider the tested code as a bootloader
    vm.tx_origin = Box::new(address_from_str_radix(DEFAULT_CALLER, 10));
    vm.push_bootloader_context(bootloader_context);
    vm.timestamp = INITIAL_TIMESTAMP;
    vm.memory_page_counter = INITIAL_MEMORY_COUNTER;

    vm
}