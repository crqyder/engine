/// Tests by parsing the biome_definitions.nbt file
#[test]
pub fn test_biome_definitions() {
    use crate::*;
    use ::binary::*;

    let bytes: &[u8; 25943] = include_bytes!("./biome_definitions.nbt");
    let mut buffer = Buffer::from(bytes.to_vec());

    let nbt = RootNBT::<NetworkLittleEndian>::deserialize(&mut buffer).unwrap();
    println!("{:?}", nbt);
}

/// Tests by parsing the biome_definitions_full.nbt file
#[test]
pub fn test_biome_definitions_full() {
    use crate::*;
    use ::binary::*;

    let bytes: &[u8; 41676] = include_bytes!("./biome_definitions_full.nbt");
    let mut buffer = Buffer::from(bytes.to_vec());

    let nbt = RootNBT::<NetworkLittleEndian>::deserialize(&mut buffer).unwrap();
    println!("{:?}", nbt);
}

/// Tests by parsing the canonical_block_states.nbt file
#[test]
pub fn test_canonical_block_states() {
    use crate::*;
    use ::binary::*;

    let bytes: &[u8; 1987768] = include_bytes!("./canonical_block_states.nbt");
    let mut buffer = Buffer::from(bytes.to_vec());

    while buffer.remaining() != 0 {
        _ = RootNBT::<NetworkLittleEndian>::deserialize(&mut buffer).unwrap();
    }
}

/// Tests by parsing the entity_identifiers.nbt file
#[test]
pub fn test_entity_identifiers() {
    use crate::*;
    use ::binary::*;

    let bytes: &[u8; 8173] = include_bytes!("./entity_identifiers.nbt");
    let mut buffer = Buffer::from(bytes.to_vec());

    let nbt = RootNBT::<NetworkLittleEndian>::deserialize(&mut buffer).unwrap();
    println!("{:?}", nbt);
}

/// Tests by parsing the crafting_data.nbt file
#[test]
pub fn test_crafting_data() {
    use crate::*;
    use ::binary::*;

    let bytes: &[u8; 623733] = include_bytes!("./crafting_data.nbt");
    let mut buffer = Buffer::from(bytes.to_vec());

    let nbt = RootNBT::<NetworkLittleEndian>::deserialize(&mut buffer).unwrap();
    println!("{:?}", nbt);
}

/// Tests by parsing the item_runtime_ids.nbt file
#[test]
pub fn test_item_runtime_ids() {
    use crate::*;
    use ::binary::*;

    let bytes: &[u8; 44130] = include_bytes!("./item_runtime_ids.nbt");
    let mut buffer = Buffer::from(bytes.to_vec());

    let nbt = RootNBT::<NetworkLittleEndian>::deserialize(&mut buffer).unwrap();
    println!("{:?}", nbt);
}

/// Tests by parsing the creative_items.nbt file
#[test]
pub fn test_creative_items() {
    use crate::*;
    use ::binary::*;

    let bytes: &[u8; 96831] = include_bytes!("./creative_items.nbt");
    let mut buffer = Buffer::from(bytes.to_vec());

    let nbt = RootNBT::<NetworkLittleEndian>::deserialize(&mut buffer).unwrap();
    println!("{:?}", nbt);
}
