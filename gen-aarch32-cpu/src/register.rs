//! Code for handling registers
//!
//! This code is used after the JSON has been parsed

use std::{fmt::Write, ops::Range};

/// Information about an Arm system register
#[derive(Debug)]
pub struct Info {
    /// The name of the register
    pub name: String,
    /// Descriptive text for the register
    pub description: Option<String>,
    /// The fields in this register
    pub fields: Vec<BitField>,
    /// How many bits in this struct
    pub width: u32,
    /// Info to read the register
    pub read_asm: Option<CoprocInfo>,
    /// Info to write the register
    pub write_asm: Option<CoprocInfo>,
}



/// How to access this co-proc system register
#[derive(Debug, PartialEq)]
pub enum CoprocInfo {
    Single {
        cp: u8,
        cr_n: u8,
        cr_m: u8,
        op1: u8,
        op2: u8
    },
    Double {
        cp: u8,
        cr_m: u8,
        op1: u8,
    }
}

/// A bitfield in a register
#[derive(Debug)]
pub struct BitField {
    /// The name of the bitfield
    pub name: String,
    /// The range of bits the field covers
    pub bit_ranges: Vec<Range<u32>>,
    /// The type for this bitfield
    pub field_type: String,
    /// How can this field be accessed
    pub access_kind: AccessKind,
}

impl BitField {
    /// Is this a single bit?
    pub fn is_single(&self) -> bool {
        self.bit_ranges.len() == 1 && self.bit_ranges[0].len() == 1
    }

    /// Get the bit ranges as a string
    pub fn bit_ranges(&self) -> String {
        if self.bit_ranges.len() == 1 {
            Self::format_bitrange(&self.bit_ranges[0])
        } else {
            let mut output = Vec::new();
            for range in self.bit_ranges.iter() {
                output.push(Self::format_bitrange(range));
            }
            format!("[{}]", output.join(", "))
        }
    }

    /// Turn a range into a string
    fn format_bitrange(range: &Range<u32>) -> String {
        let width = range.end - range.start;
        if width == 1 {
            format!("{}", range.start)
        } else {
            format!("{}..={}", range.start, range.end - 1)
        }
    }
}

/// The ways a register can be access
#[derive(Debug)]
pub enum AccessKind {
    Read,
    ReadWrite
}

impl AccessKind {
    fn to_attr(&self) -> &'static str {
        match self {
            AccessKind::Read => "r",
            AccessKind::ReadWrite => "rw",
        }
    }
}

impl Info {
    /// Generate the module definition for this register
    ///
    /// Generates text like `pub mod myregister;`
    pub fn module_definition(&self) -> Result<String, anyhow::Error> {
        Ok(format!("pub mod {};", self.module_name()))
    }

    /// Generate the module file for this register
    pub fn module_contents(&self) -> Result<String, anyhow::Error> {
        let mut buffer = String::new();
        writeln!(buffer, "//! Handles the {}", self.name_description())?;
        writeln!(buffer)?;
        writeln!(buffer, "#[allow(unused)]")?;
        writeln!(buffer, "use arbitrary_int::*;")?;
        writeln!(buffer)?;
        writeln!(buffer, "/// The {}", self.name_description())?;
        writeln!(buffer, "#[bitbybit::bitfield(u{})]", self.width)?;
        writeln!(buffer, "pub struct {} {{", self.struct_name())?;
        for field in self.fields.iter() {
            if field.is_single() {
                writeln!(buffer, "    #[bit({}, {})]", field.bit_ranges(), field.access_kind.to_attr())?;
            } else {
                writeln!(buffer, "    #[bits({}, {})]", field.bit_ranges(), field.access_kind.to_attr())?;
            }
            writeln!(buffer, "    {}: {},", field.name, field.field_type)?;
        }
        writeln!(buffer, "}}")?;
        writeln!(buffer)?;

        match self.read_asm {
            Some(CoprocInfo::Single { cp, cr_n, cr_m, op1, op2  }) => {
                writeln!(buffer, "impl crate::register::SysRegRead for {} {{", self.struct_name())?;
                writeln!(buffer, "    const CP: u32 = {};", cp)?;
                writeln!(buffer, "    const CRN: u32 = {};", cr_n)?;
                writeln!(buffer, "    const OP1: u32 = {};", op1)?;
                writeln!(buffer, "    const CRM: u32 = {};", cr_m)?;
                writeln!(buffer, "    const OP2: u32 = {};", op2)?;
                writeln!(buffer, "}}")?;
                writeln!(buffer)?;
                writeln!(buffer, "impl {} {{", self.struct_name())?;
                writeln!(buffer, "    #[inline]")?;
                writeln!(buffer, "    /// Reads {}", self.name_description())?;
                writeln!(buffer, "    pub fn read() -> Self {{")?;
                writeln!(buffer, "        unsafe {{ Self::new_with_raw_value(<Self as crate::register::SysRegRead>::read_raw()) }}")?;
                writeln!(buffer, "    }}")?;
                writeln!(buffer, "}}")?;
                writeln!(buffer)?;
            }
            Some(CoprocInfo::Double { cp, cr_m, op1  }) => {
                writeln!(buffer, "impl crate::register::SysRegRead64 for {} {{", self.struct_name())?;
                writeln!(buffer, "    const CP: u32 = {};", cp)?;
                writeln!(buffer, "    const OP1: u32 = {};", op1)?;
                writeln!(buffer, "    const CRM: u32 = {};", cr_m)?;
                writeln!(buffer, "}}")?;
                writeln!(buffer)?;                
                writeln!(buffer, "impl {} {{", self.struct_name())?;
                writeln!(buffer, "    #[inline]")?;
                writeln!(buffer, "    /// Reads {}", self.name_description())?;
                writeln!(buffer, "    pub fn read() -> Self {{")?;
                writeln!(buffer, "        unsafe {{ Self::new_with_raw_value(<Self as crate::register::SysRegRead64>::read_raw()) }}")?;
                writeln!(buffer, "    }}")?;
                writeln!(buffer, "}}")?;
                writeln!(buffer)?;
            }
            None => {
                // do nothing
            }
        }

        match self.write_asm {
            Some(CoprocInfo::Single { cp, cr_n, cr_m, op1, op2  }) => {
                writeln!(buffer, "impl crate::register::SysRegWrite for {} {{", self.struct_name())?;
                writeln!(buffer, "    const CP: u32 = {};", cp)?;
                writeln!(buffer, "    const CRN: u32 = {};", cr_n)?;
                writeln!(buffer, "    const OP1: u32 = {};", op1)?;
                writeln!(buffer, "    const CRM: u32 = {};", cr_m)?;
                writeln!(buffer, "    const OP2: u32 = {};", op2)?;
                writeln!(buffer, "}}")?;
                writeln!(buffer)?;
                writeln!(buffer)?;
                writeln!(buffer, "impl {} {{", self.struct_name())?;
                writeln!(buffer, "    #[inline]")?;
                writeln!(buffer, "    /// Writes {}", self.name_description())?;
                writeln!(buffer, "    ///")?;
                writeln!(buffer, "    /// # Safety")?;
                writeln!(buffer, "    ///")?;
                writeln!(buffer, "    /// Ensure that only valid values are written to this register. See Arm documentation for details.")?;
                writeln!(buffer, "    pub unsafe fn write(value: Self) {{")?;
                writeln!(buffer, "        unsafe {{ <Self as crate::register::SysRegWrite>::write_raw(value.raw_value()) }}")?;
                writeln!(buffer, "    }}")?;
                writeln!(buffer, "}}")?;
                writeln!(buffer)?;
            }
            Some(CoprocInfo::Double { cp, cr_m, op1  }) => {
                writeln!(buffer, "impl crate::register::SysRegWrite64 for {} {{", self.struct_name())?;
                writeln!(buffer, "    const CP: u32 = {};", cp)?;
                writeln!(buffer, "    const OP1: u32 = {};", op1)?;
                writeln!(buffer, "    const CRM: u32 = {};", cr_m)?;
                writeln!(buffer, "}}")?;
                writeln!(buffer)?;                
                writeln!(buffer, "impl {} {{", self.struct_name())?;
                writeln!(buffer, "    #[inline]")?;
                writeln!(buffer, "    /// Writes {}", self.name_description())?;
                writeln!(buffer, "    ///")?;
                writeln!(buffer, "    /// # Safety")?;
                writeln!(buffer, "    ///")?;
                writeln!(buffer, "    /// Ensure that only valid values are written to this register. See Arm documentation for details.")?;
                writeln!(buffer, "    pub unsafe fn write(value: Self) {{")?;
                writeln!(buffer, "        unsafe {{ <Self as crate::register::SysRegWrite64>::write_raw(value.raw_value()) }}")?;
                writeln!(buffer, "    }}")?;
                writeln!(buffer, "}}")?;
                writeln!(buffer)?;
            }
            None => {
                // do nothing
            }
        }

        if self.read_asm.is_some() && self.write_asm.is_some() {
            writeln!(buffer, "impl {} {{", self.struct_name())?;
            writeln!(buffer, "    #[inline]")?;
            writeln!(buffer, "    /// Modifies {}", self.name_description())?;
            writeln!(buffer, "    ///")?;
            writeln!(buffer, "    /// # Safety")?;
            writeln!(buffer, "    ///")?;
            writeln!(buffer, "    /// Ensure that only valid values are written to this register. See Arm documentation for details.")?;
            writeln!(buffer, "    pub unsafe fn modify<F>(f: F) where F: FnOnce(&mut Self) {{")?;
            writeln!(buffer, "        let mut value = Self::read();")?;
            writeln!(buffer, "        f(&mut value);")?;
            writeln!(buffer, "        unsafe {{ Self::write(value); }}")?;
            writeln!(buffer, "    }}")?;
            writeln!(buffer, "}}")?;
            writeln!(buffer)?;
        }

        Ok(buffer)
    }

    /// The name and description of this register
    pub fn name_description(&self) -> String {
        if let Some(description) = &self.description {
            format!("{} (*{}*) Register", self.name, description)
        } else {
            format!("{} Register", self.name)
        } 
    }

    /// The name of this register, but in lower case
    pub fn module_name(&self) -> String {
        self.name.to_ascii_lowercase()
    }

    /// The name of this register, but in title case
    pub fn struct_name(&self) -> String {
        let mut i = self.name.chars();
        let initial = i.next().expect("Register name should be non empty");
        let mut output = format!("{}", initial.to_ascii_uppercase());
        for ch in i {
            output.push(ch.to_ascii_lowercase());
        }
        output
    }
}
