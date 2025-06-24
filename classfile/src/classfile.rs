use crate::{classname, classfile_version, access_flags, constant_pool};

/// Represents the content of a .class file.
#[derive(Debug, Default)]
pub struct ClassFile {
    pub version: ClassFileVersion,
    pub constants: ConstantPool,
    pub flags: AccessFlags,
    pub name: ClassName,
    pub superclass: Option<ClassName>,
    pub interfaces: Vec<String>,
    pub fields: Vec<ClassFileField>,
    pub methods: Vec<ClassFileMethod>,
    pub deprecated: bool,
    pub source_file: Option<String>,
}