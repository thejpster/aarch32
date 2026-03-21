//! Code for our configuration file, and much of the processing

use std::collections::{BTreeMap, HashMap};

use anyhow::Context;

use crate::register::{AccessKind, BitField, CoprocInfo, Info};

/// JSON import configuration
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Import {
    /// the JSON file we load
    pub filename: std::path::PathBuf,
}

/// Code-gen configuration
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Generate {
    pub registers: BTreeMap<String, RegisterConfig>,
}

/// Configuration for a register
#[derive(Debug, Clone, serde::Deserialize)]
pub struct RegisterConfig {
    /// Additional descriptive text to add to a register
    pub description: Option<String>,
    /// Which fieldset to use (0 by default)
    #[serde(default)]
    pub fieldset: usize,
    /// Bitfield renamining
    pub rename: Option<Vec<Rename>>,
}

/// Information about a Bitfield rename
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Rename {
    /// Old bitfield name
    pub from: String,
    /// New bitfield name
    pub to: String,
}

/// Export configuration
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Export {
    /// Which folder to export to
    pub folder: std::path::PathBuf,
}

/// Our configuration file
#[derive(Debug, Clone, serde::Deserialize)]
pub struct TopLevel {
    pub import: Import,
    pub generate: Generate,
    pub export: Export,
}

impl TopLevel {
    /// Load the JSON given in the configuration and parse it
    pub fn parse_registers(&self) -> Result<Vec<Info>, anyhow::Error> {
        log::debug!("Reading {}...", self.import.filename.display());
        let json_str = std::fs::read_to_string(&self.import.filename).context("Loading JSON file")?;
        log::debug!("Parsing JSON...");
        let register_entries = serde_json::from_str::<Vec<arm_sysregs_json::RegisterEntry>>(&json_str)?;
        log::debug!("Parsing JSON complete, got {} entries. Now processing...", register_entries.len());
        let mut register_infos = Vec::new();
        for register_entry in register_entries {
            let registers = self.convert_entry_to_info(register_entry)?;
            register_infos.extend(registers);
        }
        log::debug!("Processing complete, got {} registers", register_infos.len());
        Ok(register_infos)
    }

    /// Convert from the native Arm format to a format that's easier for us to use
    ///
    /// We filter out any registers we don't have configuration for
    pub fn convert_entry_to_info(&self, entry: arm_sysregs_json::RegisterEntry) -> Result<Vec<Info>, anyhow::Error> {
        Ok(match entry {
            arm_sysregs_json::RegisterEntry::Register(register) => self.convert_register_to_info(register)?.into_iter().collect(),
            arm_sysregs_json::RegisterEntry::RegisterArray(register_array) => self.convert_array_to_info(register_array)?,
            arm_sysregs_json::RegisterEntry::RegisterBlock(register_block) => self.convert_block_to_info(register_block)?,
        })
    }

    /// Convert a JSON Register to a RegisterInfo object
    fn convert_register_to_info(&self, register: arm_sysregs_json::Register) -> Result<Option<Info>, anyhow::Error> {
        log::trace!("Found register {} that we want", register.name);
        let Some(register_config) = self.find_info(&register.name) else {
            return Ok(None);
        };
        log::debug!("Found register {} that we want, config {:?}", register.name, register_config);
        let mut read_asm = None;
        let mut write_asm = None;
        for accessor in register.accessors.iter() {
            if let arm_sysregs_json::Accessor::SystemAccessor(co_proc) = accessor {
                log::debug!("- Accessed with {} {:?}", co_proc.name, co_proc.encoding);
                match co_proc.name.as_str() {
                    "A32.MCR" => {
                        // write to co-processor
                        let cp = debin(&co_proc.encoding[0].encodings["coproc"].value);
                        let cr_n = debin(&co_proc.encoding[0].encodings["CRn"].value);
                        let cr_m = debin(&co_proc.encoding[0].encodings["CRm"].value);
                        let op1 = debin(&co_proc.encoding[0].encodings["opc1"].value);
                        let op2 = debin(&co_proc.encoding[0].encodings["opc2"].value);
                        write_asm = Some(CoprocInfo::Single { cp, cr_n, cr_m, op1, op2 });
                    }
                    "A32.MCRR" => {
                        // write to co-processor, 64-bit
                        let cp = debin(&co_proc.encoding[0].encodings["coproc"].value);
                        let cr_m = debin(&co_proc.encoding[0].encodings["CRm"].value);
                        let op1 = debin(&co_proc.encoding[0].encodings["opc1"].value);
                        write_asm = Some(CoprocInfo::Double { cp, cr_m, op1 });
                    }
                    "A32.MRC" => {
                        // read from co-processor
                        let cp = debin(&co_proc.encoding[0].encodings["coproc"].value);
                        let cr_n = debin(&co_proc.encoding[0].encodings["CRn"].value);
                        let cr_m = debin(&co_proc.encoding[0].encodings["CRm"].value);
                        let op1 = debin(&co_proc.encoding[0].encodings["opc1"].value);
                        let op2 = debin(&co_proc.encoding[0].encodings["opc2"].value);
                        read_asm = Some(CoprocInfo::Single { cp, cr_n, cr_m, op1, op2 });
                    }
                    "A32.MRRC" => {
                        // read from co-processor, 64-bit
                        let cp = debin(&co_proc.encoding[0].encodings["coproc"].value);
                        let cr_m = debin(&co_proc.encoding[0].encodings["CRm"].value);
                        let op1 = debin(&co_proc.encoding[0].encodings["opc1"].value);
                        read_asm = Some(CoprocInfo::Double { cp, cr_m, op1 });
                    }
                    _ => {
                        panic!("Unknown system register accessor {:?} on register {:?}", accessor, register.name);
                    }
                }
            } else {
                log::warn!("Unknown accessor {:?} on register {:?}", accessor, register.name);
            }
        }
        let (fields, width) = if register.fieldsets.is_empty() {
            // create a dummy bitfield covering the whole width
            let dummy = vec![BitField {
                name: "inner".to_string(),
                bit_ranges: vec![0..32],
                field_type: "u32".into(),
                access_kind: AccessKind::ReadWrite,
            }];
            (dummy, 32)
        } else {
            (
                self.fieldset_to_fields(&register.fieldsets[register_config.fieldset], &register.name, &register_config.rename)?,
                register.fieldsets[register_config.fieldset].width,
            )
        };
        if read_asm.is_some() || write_asm.is_some() {
            let ri = Info {
                name: register.name,
                description: register_config.description.clone(),
                fields,
                width,
                read_asm,
                write_asm
            };
            Ok(Some(ri))            
        } else {
            Ok(None)
        }
    }

    /// Convert a JSON Register Array to a RegisterInfo object
    fn convert_array_to_info(&self, register_array: arm_sysregs_json::RegisterArray) -> Result<Vec<Info>, anyhow::Error> {
        log::trace!("Found regarray {}", register_array.name);
        let mut output = Vec::new();
        let index_variable = format!("<{}>", register_array.index_variable);
        for range in register_array.indexes {
            for idx in range.start..(range.start + range.width) {
                let name = register_array.name.replace(&index_variable, &format!("{idx}"));
                if let Some(register_config) = self.find_info(&name) {
                    log::debug!("Found register {} that we want, config {:?}", name, register_config);
                    let description = register_config.description.clone();
                    let fields = self.fieldset_to_fields(&register_array.fieldsets[register_config.fieldset], &name, &register_config.rename)?;
                    let ri = Info {
                        name,
                        description,
                        fields,
                        width: register_array.fieldsets[register_config.fieldset].width,
                        read_asm: None,
                        write_asm: None
                    };
                    output.push(ri);
                }
            }
        }
        Ok(output)
    }

    /// Convert a JSON Register Block to a RegisterInfo object
    fn convert_block_to_info(&self, register_block: arm_sysregs_json::RegisterBlock) -> Result<Vec<Info>, anyhow::Error> {
        log::trace!("Found regblock {}", register_block.name);
        if self.find_info(&register_block.name).is_none() {
            return Ok(Vec::new());
        }
        log::debug!("Found regblock {} that we want", register_block.name);
        Ok(Vec::new())
    }

    /// Get the description for a register, if one exists
    fn find_info(&self, name: &str) -> Option<&RegisterConfig> {
        self.generate.registers.get(name)
    }

    /// Convert a JSON fieldset to a Vec of Rust Bitfields
    fn fieldset_to_fields(&self, fieldset: &arm_sysregs_json::Fieldset, register_name: &str, rename: &Option<Vec<Rename>>) -> Result<Vec<BitField>, anyhow::Error> {
        let mut fields = Vec::new();
        let mut anon_idx = 0;
        let mut rename_map = HashMap::new();
        if let Some(rename_vec) = rename {
            for Rename { from, to } in rename_vec.iter() {
                log::debug!("Will rename {reg}.{from} to {reg}.{to}", reg = register_name);
                rename_map.insert(from, to);
            }
        }
        for field_entry in fieldset.values.iter() {
            match field_entry {
                arm_sysregs_json::FieldEntry::Field(field) => {
                    let bitfield = self.field_to_field(&mut anon_idx, field.name.as_deref(), &field.rangeset, AccessKind::ReadWrite)?;
                    fields.push(bitfield);
                }
                arm_sysregs_json::FieldEntry::ConstantField(field) => {
                    let bitfield = self.field_to_field(&mut anon_idx, field.name.as_deref(), &field.rangeset, AccessKind::Read)?;
                    fields.push(bitfield);
                }
                arm_sysregs_json::FieldEntry::ImplementationDefined(field) => {
                    let bitfield = self.field_to_field(&mut anon_idx, field.name.as_deref(), &field.rangeset, AccessKind::ReadWrite)?;
                    fields.push(bitfield);
                }
                arm_sysregs_json::FieldEntry::Reserved(_field) => {
                    // we can ignore reserved fields
                }
                arm_sysregs_json::FieldEntry::Array(field) => {
                    let Some(field_name) = &field.name else {
                        anyhow::bail!("Array with anonymous field name?");
                    };
                    for (idx, range) in field.indexes.iter().enumerate() {
                        let name = field_name.replace(&format!("<{}>", field.index_variable), &format!("{}", idx));
                        let bitfield = self.field_to_field(&mut anon_idx, Some(&name), &[range.clone()], AccessKind::ReadWrite)?;
                        fields.push(bitfield);
                    }
                }
                arm_sysregs_json::FieldEntry::Vector(_field) => {
                    anyhow::bail!("Cannot handle Vector in {}", register_name)
                }
                arm_sysregs_json::FieldEntry::ConditionalField(_field) => {
                    log::warn!("Cannot handle ConditionalField in {}", register_name)
                }
                arm_sysregs_json::FieldEntry::Dynamic(_field) => {
                    log::warn!("Cannot handle Dynamic in {}", register_name)
                }
            }
        }
        for field in fields.iter_mut() {
            if let Some(new_name) = rename_map.get(&field.name) {
                log::info!("Renaming {reg}.{from} to {reg}.{to}", reg = register_name, from = field.name, to = new_name);
                field.name = new_name.to_string();
            }
        }
        Ok(fields)
    }

    /// Convert a JSON field to a BitField
    fn field_to_field(&self, index: &mut i32, field_name: Option<&str>, rangeset: &[arm_sysregs_json::Range], access_kind: AccessKind) -> Result<BitField, anyhow::Error> {
        let placeholder = format!("anon{}", index);
        let field_name = field_name.unwrap_or_else(|| {
            *index += 1;
            &placeholder
        });
        let name = field_name.to_ascii_lowercase();
        let mut bit_ranges = Vec::new();
        let mut width = 0;
        for range in rangeset.iter().rev() {
            let bit_range = range.start..(range.start + range.width);
            width += range.width;
            bit_ranges.push(bit_range);
        }
        let bitfield = BitField {
            name,
            bit_ranges,
            field_type: if width == 1 { "bool".to_string() } else { format!("u{}", width) },
            access_kind,
        };
        Ok(bitfield)
    }
}

/// Convert binary string "'0001'" into the number 1
fn debin(input: &str) -> u8 {
    let Some(i) = input.strip_prefix('\'') else {
        panic!("Bad hex {input}");
    };
    let Some(i) = i.strip_suffix('\'') else {
        panic!("Bad hex {input}");
    };
    let mut output = 0;
    for ch in i.chars() {
        match ch {
            '0' => {
                output <<= 1;
            }
            '1' => {
                output <<= 1;
                output |= 1;
            }
            _ => {
                panic!("Bad hex {input}");
            }
        }
    }
    output
}
