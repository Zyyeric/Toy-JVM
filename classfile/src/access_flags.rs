bitflags!{
    pub struct AccessFlags: u16 {
        // Declared public; may be accessed from outside its package
        const PUBLIC = 0x0001;
        // Declared final; no subclasses allowed.
        const FINAL = 0x0010;
        // Treat superclass methods specially when invoked by the *invokespecial* instruction.
        const SUPER = 0x0020;
        // Is an interface, not a class.
        const INTERFACE = 0x0200;
        // Declared abstract; must not be instantiated.
        const ABSTRACT = 0x0400;
        // Declared synthetic; not present in the source code.
        const SYNTHETIC = 0x1000;
        // Declared as an annotation type.
        const ANNOTATION = 0x2000;
        // Declared as an enum type.
        const ENUM = 0x4000;
    }
}