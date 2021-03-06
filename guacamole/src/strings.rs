use rand::{distributions::Alphanumeric, Rng};

use super::Guac;
use super::Guacamole;

////////////////////////////////////////// StringGuacamole /////////////////////////////////////////

pub trait StringGuacamole: std::fmt::Debug {
    fn guacamole(&self, guac: &mut Guacamole) -> String;
}

impl Guac<String> for dyn StringGuacamole {
    fn guacamole(&self, guac: &mut Guacamole) -> String {
        StringGuacamole::guacamole(self, guac)
    }
}

////////////////////////////////////// IndependentStringLength /////////////////////////////////////

pub trait IndependentStringLength: std::fmt::Debug {
    fn guacamole(&self, guac: &mut Guacamole) -> usize;
}

impl Guac<usize> for dyn IndependentStringLength {
    fn guacamole(&self, guac: &mut Guacamole) -> usize {
        IndependentStringLength::guacamole(self, guac)
    }
}

////////////////////////////////////// IndependentStringSelect /////////////////////////////////////

pub trait IndependentStringSelect: std::fmt::Debug {
    fn guacamole(&self, guac: &mut Guacamole) -> u64;
}

impl Guac<u64> for dyn IndependentStringSelect {
    fn guacamole(&self, guac: &mut Guacamole) -> u64 {
        IndependentStringSelect::guacamole(self, guac)
    }
}

//////////////////////////////////////// IndependentStrings ////////////////////////////////////////

#[derive(Debug)]
pub struct IndependentStrings {
    pub length: Box<dyn IndependentStringLength>,
    pub select: Box<dyn IndependentStringSelect>,
}

impl StringGuacamole for IndependentStrings {
    fn guacamole(&self, guac: &mut Guacamole) -> String {
        let length = self.length.guacamole(guac);
        let select = self.select.guacamole(guac);
        let inner = Guacamole::new(select);
        inner.sample_iter(&Alphanumeric).take(length).map(char::from).collect()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////// Concrete types //////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////// ConstantLength //////////////////////////////////////////

#[derive(Debug)]
pub struct ConstantLength {
    pub constant: usize,
}

impl IndependentStringLength for ConstantLength {
    fn guacamole(&self, _: &mut Guacamole) -> usize {
        self.constant
    }
}

/////////////////////////////////////////// RandomSelect ///////////////////////////////////////////

#[derive(Debug)]
pub struct RandomSelect {}

impl IndependentStringSelect for RandomSelect {
    fn guacamole(&self, guac: &mut Guacamole) -> u64 {
        guac.gen()
    }
}

/////////////////////////////////////////////// tests //////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_random_0xf00d() {
        let length = ConstantLength {
            constant: 8,
        };
        let select = RandomSelect {};
        let gen = IndependentStrings {
            length: Box::new(length),
            select: Box::new(select),
        };
        let mut guac = Guacamole::new(0xf00d);
        let s: String = gen.guacamole(&mut guac);
        assert_eq!(s, "ZKxPwt3j");
    }

    #[test]
    fn constant_random_0x1eaf() {
        let length = ConstantLength {
            constant: 8,
        };
        let select = RandomSelect {};
        let gen = IndependentStrings {
            length: Box::new(length),
            select: Box::new(select),
        };
        let mut guac = Guacamole::new(0x1eaf);
        let s: String = gen.guacamole(&mut guac);
        assert_eq!(s, "bUmn3nxd");
    }

    #[test]
    fn constant_random_0xc0ffee() {
        let length = ConstantLength {
            constant: 8,
        };
        let select = RandomSelect {};
        let gen = IndependentStrings {
            length: Box::new(length),
            select: Box::new(select),
        };
        let mut guac = Guacamole::new(0xc0ffee);
        let s: String = gen.guacamole(&mut guac);
        assert_eq!(s, "g2Hhyjiy");
    }

    #[test]
    fn constant_random_0xcafe() {
        let length = ConstantLength {
            constant: 8,
        };
        let select = RandomSelect {};
        let gen = IndependentStrings {
            length: Box::new(length),
            select: Box::new(select),
        };
        let mut guac = Guacamole::new(0xcafe);
        let s: String = gen.guacamole(&mut guac);
        assert_eq!(s, "AkryThCW");
    }
}
